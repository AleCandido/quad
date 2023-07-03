use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

use quad::constants::Myf64;
use quad::qag::Qag;
use quad::errors::*;

#[pyfunction]
fn qag_vec(
    ob: &PyAny,
    a: f64,
    b: f64,
    epsabs: Option<f64>,
    epsrel: Option<f64>,
    key: Option<i32>,
    limit: Option<usize>,
    points: Option<Vec<f64>>,
    more_info: Option<bool>,
) -> PyResult<QagsResult> {
    let pointss = {
        if points.is_some() {
            points.unwrap()
        } else {
            [0.0; 0].to_vec()
        }
    };
    let limitt = {
        if limit.is_some() {
            limit.unwrap()
        } else {
            50
        }
    };
    let keyy = {
        if key.is_some() {
            key.unwrap()
        } else {
            2
        }
    };
    let epsabss = {
        if epsabs.is_some() {
            epsabs.unwrap()
        } else {
            1.49e-8
        }
    };
    let epsrell = {
        if epsrel.is_some() {
            epsrel.unwrap()
        } else {
            1.49e-8
        }
    };
    let more_infoo = {
        if more_info.is_some() {
            more_info.unwrap()
        } else {
            false
        }
    };

    let qag = Qag {
        key: keyy,
        limit: limitt,
        points: pointss,
        more_info: more_infoo,
    };
    let f = |x: f64| lambda_eval(ob, x);
    let res = qag.qintegrate(&f, a, b, epsabss, epsrell);
    if res.is_err(){
        match res.unwrap_err(){
            QagError::Invalid => return Err(PyErr::new::<PyTypeError, _>(INVALID_ERROR_MESSAGE)),
            QagError::MaxIteration => return Err(PyErr::new::<PyTypeError, _>(MAX_ITERATION_ERROR_MESSAGE)),
            QagError::BadTolerance => return Err(PyErr::new::<PyTypeError, _>(BAD_TOLERANCE_ERROR_MESSAGE)),
            QagError::BadFunction => return Err(PyErr::new::<PyTypeError, _>(BAD_FUNCTION_ERROR_MESSAGE)),
            QagError::Diverge => return Err(PyErr::new::<PyTypeError, _>(DIVERGE_ERROR_MESSAGE)),
        }
    }
    let res = res.unwrap();
    let (result, abserr, more_inf) = (res.result, res.abserr, res.more_info);
    if more_inf.is_none() {
            return Ok(QagsResult {
                result,
                abserr,
                more_info: None,
            },
        )
    } else {
        let mut more_inf_py: Vec<(f64, f64, f64, Vec<f64>)> = vec![];
        let more_inf_unwrapped = more_inf.unwrap();
        let (mut hash, mut heap) = (more_inf_unwrapped.hash, more_inf_unwrapped.heap);
        let (neval, last) = (more_inf_unwrapped.neval, more_inf_unwrapped.last);
        for _k in 0..heap.len() {
            let old_interval = heap.pop().unwrap();
            let ((x, y), old_err) = (old_interval.interval, old_interval.err);
            let old_res = hash.remove(&(Myf64 { x }, Myf64 { x: y })).unwrap();
            more_inf_py.push((x, y, old_err, old_res));
        }
        Ok(QagsResult {
                result,
                abserr,
                more_info: Some((neval, last, more_inf_py)),
            },
        )
    }
}

fn lambda_eval(ob: &PyAny, z: f64) -> Vec<f64> {
    let f = |x: f64| {
        let y = (x,);
        ob.call1(y).unwrap().extract::<Vec<f64>>().unwrap()
    };
    f(z)
}

#[pyclass]
struct QagsResult {
    #[pyo3(get, set)]
    pub result: Vec<f64>,
    #[pyo3(get, set)]
    pub abserr: f64,
    #[pyo3(get, set)]
    pub more_info: Option<(i32, usize, Vec<(f64, f64, f64, Vec<f64>)>)>,
}

#[pymodule]
fn quad(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(qag_vec, m)?)?;
    Ok(())
}
