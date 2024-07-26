use pyo3::prelude::*;
mod delauney;
mod linalg;

//NEEDS TO ADD MORE SLACK TO GET CONVEX HULL
#[pyfunction]
#[pyo3(signature = (data,n_points=1000, parallel=true))]
fn delaunay(data: Vec<Vec<f64>>, n_points: usize, parallel: bool) -> PyResult<Vec<(usize, usize)>> {
    if parallel {
        Ok(delauney::core_loop(data, n_points))
    } else {
        Ok(delauney::core_loop_parallel(data, n_points))
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn trisect(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(delaunay, m)?)?;
    Ok(())
}
