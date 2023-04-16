# Dict picker (dictionary picker)

Retrieve data from Python dictionaries and lists.

## Installation

With **pip**:

```shell
pip install -U dict-picker
```

Or with **conda**:

```shell
conda install "dict-picker" -c conda-forge
```

## Usage

### `pick_by_path`

The `pick_by_path` function is used to extract values from nested dictionaries using a path-like string to identify the desired value. The function takes three arguments: a dictionary object, a string representing the path to the desired value and optionally a delimiter.

The path string consists of a sequence of keys, separated by the delimiter character (`'/'` by default), where each key represents a level in the nested dictionary. If the key has the value `'*'`, it matches any key on that level, or sets it to check all items in the list.

The function returns the value at the end of the path or a list of values if the path contains a wildcard. If the path is not found, the function returns `None`.

```python
from dict_picker import pick_by_path

example_dict = {
    "foo": "bar",
    "baz": {
        "qux": "quux",
        "quuux": "corge"
    },
    "fred": {
        "wilma": "betty",
        "barney": "pebbles"
    },
    "arr": [
        { 'id': 123 },
        { 'id': 456 },
        { 'id': 789 },
        { 'name': 'bubbles' },
    ],
}

assert pick_by_path(example_dict, "foo") == "bar"
assert pick_by_path(example_dict, "baz/qux") == "quux"
assert pick_by_path(example_dict, "fred/wilma") == "betty"
assert pick_by_path(example_dict, "*/qux") == "quux"
assert pick_by_path(example_dict, "fred/*") == {"wilma": "betty", "barney": "pebbles"}
assert pick_by_path(example_dict, "*/quux") is None
assert pick_by_path(example_dict, "arr/*/id") == [123, 456, 789]

# slice syntax
assert pick_by_path(example_dict, "arr/0") == {'id': 123}
assert pick_by_path(example_dict, "arr/1/id") == 456
assert pick_by_path(example_dict, "arr/-1") == {'name': 'bubbles'}
assert pick_by_path(example_dict, "arr/1:/id") == [456, 789, None]
assert pick_by_path(example_dict, "arr/::-2/id") == [789, 123]
```

Parameters:

- `obj` -- A dictionary object to search for the desired value.
- `path: str` -- A string representing the path to the desired value.
- `delimiter: str` -- A string used to separate keys in the path string. Default is "/".

Return value:

- Returns the value found at the end of the path string, or a list of values if the path contains a wildcard. If the path is not found, None is returned.


### `pick_by_paths`

The `pick_by_paths` function is similar to `pick_by_path`, but can extract values from multiple paths at once. It takes a dictionary object and a list of path strings as arguments, and returns a list of values found at the end of each path string or None if a path is not found. This works faster than running `pick_by_path` in a loop.

```python
from dict_picker import pick_by_path, pick_by_paths

example_dict = {
    "foo": "bar",
    "baz": {
        "qux": "quux",
        "quuux": "corge"
    },
    "fred": {
        "wilma": "betty",
        "barney": "pebbles"
    },
    "arr": [
        { 'id': 123 },
        { 'id': 456 },
        { 'id': 789 },
        { 'name': 'bubbles' },
    ],
}

assert pick_by_paths(example_dict, ["fred/*","*/quux", "arr/*/id",]) == [{"wilma": "betty", "barney": "pebbles"}, None, [123, 456, 789]]
```

## Build from sources

### Cargo

```shell
curl --proto '=https' --tlsv1.2 -sSf <https://sh.rustup.rs> | sh
```

For other installation options, see [the official website](https://www.rust-lang.org/tools/install).

### Maturin

To bind python and rust, pyo3 is used. The [Maturin](https://github.com/PyO3/maturin) library is used to make it easy to work with.

```shell
python -m pip install maturin
```

### Build

```shell
python -m maturin build --release
```

Wheel will be released under the system and python in which it will be built. [Read more about the compilation](https://www.maturin.rs/distribution.html).

The finished wheel can be found in the target/wheels folder.

## Build with docker

```shell
docker run --rm -v $(pwd):/io ghcr.io/pyo3/maturin build --release  # or other maturin arguments
```

On macOS, it is better to use docker instead of podman.

## Roadmap

- [ ] Arbitrary levels skip operator `**`;
- [X] Search inside an array of dictionaries. For example:

    ```python
    {
        arr:  [
            { id: 123 },
            { id: 456 },
            { id: 789 },
        ]
    }
    'arr/*/id' -> [123, 456, 789]
    ```

- [X] Search by tuple of patterns. Returns the result as a list of found values.

- [X] add option to access lists via python slice syntax ([issue #2](https://github.com/sarvensis/dict-picker/issues/2)):

    ```python
    {
        arr:  [
            { id: 123 },
            { id: 456 },
            { id: 789 },
        ]
    }
    'arr/1/id' -> 456
    'arr/1' -> { id: 123 }
    'arr/:2/id' -> [123, 456]
    'arr/1:' -> [{ id: 456 },{ id: 789 }]
    'arr/::2/id' -> [123, 789]
    ```
