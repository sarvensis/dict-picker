# tests

To run tests during development, install a virtual environment and pytest. After making changes to the code, run the build, install the package in the environment, and run the tests:

```shell
maturin develop && pip install -e . && python -m pytest
```
