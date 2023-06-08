# Python with Rust

## References
* https://github.com/PyO3/pyo3
* https://www.maturin.rs/tutorial.html

## When starting from scratch

Install pyenv using [this guide](https://github.com/pyenv/pyenv#installation).

If you have it already installed or newly installed run

    pyenv update

Then install the python version of this project using

    pyenv install 3.10.8

Now you should have the required python version.

Create the environment

    mkdir python-with-rust
    cd python-with-rust
    make venv
    source .venv/bin/activate
    pip install maturin

Initialize stuff with `maturin`

    maturin init

Open `pyproject.toml` and add your dependencies, including `maturin`. Then

    make compile
    make install

Now you should have a working minimal python environment.

Running

    make build

Should compile your rust library so it can be used from python.

Everything worked as expected if

    pytest -vx .

Returns

    tests/test_test.py::test_sum_as_string_py PASSED                                   [ 50%]
    tests/test_test.py::test_sum_as_string_rs PASSED                                   [100%]


## When starting with stuff in the repo

    make venv
    make install
    make build
    make test

Should return in the end

    tests/test_test.py::test_sum_as_string_py PASSED                                   [ 50%]
    tests/test_test.py::test_sum_as_string_rs PASSED                                   [100%]