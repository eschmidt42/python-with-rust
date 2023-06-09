# Python with Rust

## References
* https://github.com/PyO3/pyo3
* https://www.maturin.rs/tutorial.html

## When starting from scratch

### Setup

Install pyenv using [this guide](https://github.com/pyenv/pyenv#installation).

If you have it already installed or newly installed run

    pyenv update

Then install the python version of this project using

    pyenv install 3.10.8


Now you should have the required python version installed.

To create the virtual environment

    mkdir python-with-rust
    cd python-with-rust
    pyenv local 3.10.8
    make venv
    source .venv/bin/activate
    pip install maturin

Initialize stuff with `maturin`

    maturin init

### Creating python and rust functionality 

To be able to develop and use python and rust in parallel in one package we create the same function one time in python and one time in rust and then run pytest.

Now let's create a dummy function in python changing the init to

```python
# python_with_rust/__init__.py
from ._rust import *

def sum_as_string_py(a:int,b:int) -> str:
    return f"{a+b}"
```

`src/lib.rs` we'll modify slightly to

```rust
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string_rs(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}
```

### Telling maturin / rust / python where is what

This is the part that took me a good few hours to figure out.

Step 1: In your `python_with_rust/__init__.py` you will want to have 
```python
from ._rust import *
```

What the heck is `_rust`? Good question. This contains our rust stuff and will be created with `maturin`.

Step 2: For maturin to create a shared object we set `module-name` in `pyproject.toml` like
```toml
[tool.maturin]
bindings = "pyo3"
features = ["pyo3/extension-module"]
module-name = "python_with_rust._rust" # <-- not added by default
```

and step 3: in `src/lib.rs` we'll want something like
```rust
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string_rs(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_rust")] // <-- not added by default, notice _rust? :P
fn python_with_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string_rs, m)?)?; // <-- also added changed the default sum_as_string to our sum_as_string_rs
    Ok(())
}
```

### Building the rust part

Okay. Now you should be able to run

    maturin develop

And then if you enter `python` in your command line and import `pwr`

```text
>>> import pwr
>>> pwr.sum_as_string_py
<function sum_as_string_py at 0x7f92a06d4310>
>>> pwr.sum_as_string_rs
<built-in function sum_as_string_rs>
```

### Running pytest

Step 1: create the tests in `tests/test_test.py` like
```python
# tests/test_test.py
import python_with_rust as pwr

def test_sum_as_string_py():
    a = 1
    b = 2

    res = pwr.sum_as_string_py(a,b)
    assert res == "3"

def test_sum_as_string_rs():
    a = 1
    b = 2

    res = pwr.sum_as_string_rs(a,b)
    assert res == "3"
```

Step 2: install pytest

    pip install pytest

Step 3: add to the end of `pyproject.toml`
```text
[tool.pytest.ini_options]
pythonpath = ["python_with_rust"]
testpaths = ["tests"]
python_files = ["test_*.py"]
```

Step 4: run

    pytest -vx

Now you should get

    tests/test_test.py::test_sum_as_string_py PASSED                                   [ 50%]
    tests/test_test.py::test_sum_as_string_rs PASSED                                   [100%]


## When starting with stuff in the repo

    make install
    make build
    make test

Should return in the end

    tests/test_test.py::test_sum_as_string_py PASSED                                   [ 50%]
    tests/test_test.py::test_sum_as_string_rs PASSED                                   [100%]