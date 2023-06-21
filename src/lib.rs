use numpy::ndarray::{ArrayD, ArrayViewD, ArrayViewMutD};
use numpy::{IntoPyArray, PyArrayDyn, PyReadonlyArrayDyn};
use pyo3::prelude::*;
use std::collections::HashMap;

/// Formats the sum of two numbers as string.
#[pyfunction(name = "sum_as_string_rs")] // name of the function once imported in Python
fn sum_as_string_py(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction(name = "a_lot_of_sums_as_string_rs")] // name of the function once imported in Python
fn a_lot_of_sums_as_string_py(a: Vec<usize>, b: Vec<usize>) -> PyResult<Vec<String>> {
    let capacity: usize = a.len();
    let mut res: Vec<String> = Vec::with_capacity(capacity);
    for i in 0..capacity {
        res.push((a[i] + b[i]).to_string());
    }
    Ok(res)
}

/// Function processing numpy arrays based on https://github.com/PyO3/rust-numpy
fn axpy(a: f64, x: ArrayViewD<'_, f64>, y: ArrayViewD<'_, f64>) -> ArrayD<f64> {
    a * &x + &y
}

#[pyfunction(name = "axpy_rs")] // name of the function once imported in Python
fn axpy_py<'py>(
    py: Python<'py>,
    a: f64,
    x: PyReadonlyArrayDyn<f64>,
    y: PyReadonlyArrayDyn<f64>,
) -> &'py PyArrayDyn<f64> {
    let x = x.as_array();
    let y = y.as_array();
    let z = axpy(a, x, y);
    z.into_pyarray(py)
}

/// equivalent of gini_py
#[pyfunction(name = "gini_rs")] // name of the function once imported in Python
fn gini_py<'py>(
    py: Python<'py>,
    categorical_values: PyReadonlyArrayDyn<i64>,
) -> PyResult<f64> {
    let x: Vec<i64> = categorical_values.clone().to_vec()?;
    let n: usize = x.len();
    // count the number of occurences of distinct values in x
    // x.into_iter().counts();
    let mut frequencies: HashMap<i64,usize> = HashMap::new();
    for value in x {
        let count = frequencies.entry(value).or_insert(0);
        *count += 1;
    }
    let mut gini: f64 = 0.;
    for (_, count) in frequencies {
        let p = count as f64 / n as f64;
        gini += p * (1.0 - p);
    }
    Ok(gini)
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_rust")]
fn python_with_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string_py, m)?)?;
    m.add_function(wrap_pyfunction!(a_lot_of_sums_as_string_py, m)?)?;
    m.add_function(wrap_pyfunction!(axpy_py, m)?)?;
    m.add_function(wrap_pyfunction!(gini_py, m)?)?;
    Ok(())
}
