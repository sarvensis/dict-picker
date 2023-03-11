use std::error::Error;

use pyo3::{
    ffi::Py_None,
    prelude::*,
    types::{PyDict, PyList},
};

fn find_in_list<'a>(list: &'a PyList, keys: &[&str]) -> Option<&'a PyAny> {
    let result_list = PyList::empty(list.py());
    for item in list {
        if let Ok(current) = item.downcast::<PyDict>() {
            if let Some(result) = detect(&current, &keys) {
                result_list.append(result).unwrap();
            }
        }
    }
    Some(&result_list)
}

fn parse_slise<'a>(key: &'a str, length: usize) -> Result<(usize, usize, i32), Box<dyn Error>> {
    let mut iter = key.split(":");

    let start = iter.next().unwrap_or("");
    let end = iter.next().unwrap_or("");
    let step = iter.next().unwrap_or("");

    let start_parsed = if start.is_empty() { 0 } else { start.parse()? };
    let end_parsed = if end.is_empty() { length } else { end.parse()? };
    let step_parsed: i32 = if step.is_empty() { 1 } else { step.parse()? };
    if step_parsed == 0 {
        panic!("Invalid step");
    }
    Ok((start_parsed, end_parsed, step_parsed))
}

fn flist<'a>(list: &'a PyList, keys: &[&str]) -> Option<&'a PyAny> {
    let key = keys.first().unwrap_or(&"");
    // check if key is a number (element position)
    if let Ok(elem_pos) = key.parse::<i32>() {
        let real_elem_position: i32;
        // if element position is negative, calculate real position
        if elem_pos < 0 {
            real_elem_position = list.len() as i32 + elem_pos;
        } else {
            real_elem_position = elem_pos;
        }

        if let Ok(value) = list.get_item(real_elem_position as usize) {
            if keys.len() == 1 {
                return Some(&value);
            }
            return detect(&value, &keys[1..]);
        }
        return None;
    }
    // check if key is a slice
    if let Ok((start, end, step)) = parse_slise(key, list.len()) {
        let result_list = PyList::empty(list.py());

        if step < 0 {
            let mut iterator = (start..end).step_by(step.abs() as usize).rev();
            while let Some(index) = iterator.next() {
                if let Ok(item) = list.get_item(index) {
                    if keys.len() == 1 {
                        result_list.append(&item).unwrap();
                    } else {
                        let result_item = detect(&item, &keys[1..]);
                        result_list.append(result_item).unwrap();
                    }
                }
            }
        } else {
            for (index, item) in list.iter().enumerate() {
                if index >= start && index < end && index % (step as usize) == 0 {
                    if keys.len() == 1 {
                        result_list.append(&item).unwrap();
                    } else {
                        let result_item = detect(&item, &keys[1..]);
                        result_list.append(result_item).unwrap();
                    }
                }
            }
        }
        return Some(&result_list);
    }
    // check if key is a wildcard
    match *key {
        "*" => {
            if keys.len() == 1 {
                return Some(&list);
            }
            find_in_list(&list, &keys[1..])
        }
        _ => None,
    }
}

fn find_in_dict<'a>(dict: &'a PyDict, keys: &[&str]) -> Option<&'a PyAny> {
    let key = keys.first().unwrap_or(&"");
    match *key {
        "*" => return fdict(&dict, &keys),
        "" => return None,
        _ => {
            if let Some(value) = dict.get_item(key) {
                if keys.len() == 1 {
                    return Some(&value);
                }
                return detect(&value, &keys[1..]);
            }
        }
    }
    None
}

fn fdict<'a>(dict: &'a PyDict, keys: &[&str]) -> Option<&'a PyAny> {
    let key = keys.first().unwrap_or(&"");
    match *key {
        "*" => {
            if keys.len() == 1 {
                return Some(&dict);
            }
            for (_, value) in dict.iter() {
                if let Ok(child_dict) = value.downcast::<PyDict>() {
                    if let Some(result) = find_in_dict(&child_dict, &keys[1..]) {
                        return Some(&result);
                    }
                }
            }
            None
        }
        "" => None,
        _ => find_in_dict(&dict, &keys),
    }
}

fn detect<'a>(elem: &'a PyAny, keys: &[&str]) -> Option<&'a PyAny> {
    match elem.get_type().name() {
        Ok("dict") => fdict(elem.downcast::<PyDict>().unwrap(), keys),
        Ok("list") => flist(elem.downcast::<PyList>().unwrap(), keys),
        _ => None,
    }
}

#[pyfunction]
fn search_in_list<'a>(dict: &PyDict, keys: Vec<&str>) -> PyResult<Option<PyObject>> {
    if let Some(result) = detect(&dict, &keys) {
        return Ok(Some(result.to_object(dict.py())));
    }
    Ok(None)
}

#[pyfunction]
fn bulk_search_vec<'a>(dict: &PyDict, task_list: Vec<Vec<&str>>) -> PyResult<Py<PyList>> {
    let result_array = PyList::empty(dict.py());
    for key_list in task_list {
        if let Some(result) = detect(&dict, &key_list) {
            result_array
                .append(Some(result.to_object(dict.py())))
                .unwrap();
        }
    }
    Ok(result_array.into_py(dict.py()))
}

#[pyfunction]
fn pick_by_path<'a>(
    dict: &PyDict,
    path: &str,
    pattern: Option<&str>,
) -> PyResult<Option<PyObject>> {
    let pattern = pattern.unwrap_or("/");
    Ok(search_in_list(
        dict,
        path.split(pattern).collect::<Vec<&str>>(),
    )?)
}

#[pyfunction]
fn pick_by_paths<'a>(
    dict: &PyDict,
    paths: Vec<&str>,
    pattern: Option<&str>,
) -> PyResult<Py<PyList>> {
    let pattern = pattern.unwrap_or("/");
    let result_array = PyList::empty(dict.py());
    for key_str in paths {
        let result = detect(&dict, &key_str.split(pattern).collect::<Vec<&str>>());
        result_array.append(result).unwrap();
    }
    Ok(result_array.into_py(dict.py()))
}

#[pymodule]
fn dict_picker(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pick_by_path, m)?)?;
    m.add_function(wrap_pyfunction!(search_in_list, m)?)?;
    m.add_function(wrap_pyfunction!(pick_by_paths, m)?)?;
    m.add_function(wrap_pyfunction!(bulk_search_vec, m)?)?;
    Ok(())
}
