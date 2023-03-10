use pyo3::{
    prelude::*,
    types::{PyDict, PyList},
};

// New version

fn find_in_list<'a>(list: &'a PyList, keys: &[&str]) -> Option<&'a PyAny> {
    let result_list = PyList::empty(list.py());
    for item in list {
        if let Ok(current) = item.downcast::<PyDict>() {
            if let Some(result) = fstart(&current, &keys) {
                result_list.append(result).unwrap();
            }
        }
    }
    Some(&result_list)
}

fn flist<'a>(list: &'a PyList, keys: &[&str]) -> Option<&'a PyAny> {
    let key = keys.first().unwrap_or(&"");
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
                return fstart(&value, &keys[1..]);
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

fn fstart<'a>(elem: &'a PyAny, keys: &[&str]) -> Option<&'a PyAny> {
    match elem.get_type().name() {
        Ok("dict") => fdict(elem.downcast::<PyDict>().unwrap(), keys),
        Ok("list") => flist(elem.downcast::<PyList>().unwrap(), keys),
        _ => None,
    }
}

#[pyfunction]
fn search_vec<'a>(dict: &PyDict, keys: Vec<&str>) -> PyResult<Option<PyObject>> {
    if let Some(result) = fstart(&dict, &keys) {
        return Ok(Some(result.to_object(dict.py())));
    }
    Ok(None)
}

#[pyfunction]
fn bulk_search_vec<'a>(dict: &PyDict, task_list: Vec<Vec<&str>>) -> PyResult<Py<PyList>> {
    let result_array = PyList::empty(dict.py());
    for key_list in task_list {
        if let Some(result) = fstart(&dict, &key_list) {
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
    Ok(search_vec(
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
        let result = fstart(&dict, &key_str.split(pattern).collect::<Vec<&str>>());
        result_array.append(result).unwrap();
    }
    Ok(result_array.into_py(dict.py()))
}

#[pymodule]
fn dict_picker(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pick_by_path, m)?)?;
    m.add_function(wrap_pyfunction!(search_vec, m)?)?;
    m.add_function(wrap_pyfunction!(pick_by_paths, m)?)?;
    m.add_function(wrap_pyfunction!(bulk_search_vec, m)?)?;
    Ok(())
}
