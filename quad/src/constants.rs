use crate::semi_infinite_function::{double_infinite_function, semi_infinite_function};
use std::cmp::Ordering;
use std::hash;
use std::sync::Arc;

#[derive(Clone)]
pub struct FnVec<'a> {
    pub components: Arc<dyn Fn(f64) -> Vec<f64> + Send + Sync + 'a>,
}

pub const EPMACH: f64 = f64::EPSILON; // the largest relative spacing.
pub const UFLOW: f64 = f64::MIN_POSITIVE; // the smallest positive magnitude.
                                          //pub const OFLOW : f64 = f64::MAX;               // oflow is the largest positive magnitude.

pub fn norm_vec(v: &[f64]) -> f64 {
    let mut norm = 0.0;
    for comp in v {
        norm += comp.powi(2);
    }
    norm = norm.sqrt();
    norm
}

pub fn res_update(v: &mut [f64], w: &[f64], z: &[f64], y: &[f64]) {
    for k in 0..v.len() {
        v[k] += w[k] + z[k] - y[k];
    }
}

pub fn add_res(v: &mut [f64], w: &[f64]) {
    for k in 0..v.len() {
        v[k] += w[k];
    }
}

pub fn points_transformed(mut points: Vec<f64>, a: f64, b: f64) -> Vec<f64> {
    points.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut points_transformed = vec![0.0; 0];
    if b == f64::INFINITY && a.is_finite() {
        for point in &points {
            points_transformed.push(1.0 / (*point - a + 1.0));
        }
    } else if a == f64::NEG_INFINITY && b.is_finite() {
        for point in &points {
            points_transformed.push(1.0 / (b - *point + 1.0));
        }
    }
    if a == f64::NEG_INFINITY && b == f64::INFINITY {
        for point in &points {
            points_transformed.push(point.signum() / (point.abs() + 1.0));
        }
    }
    points_transformed
}

#[derive(Debug, Clone)]
pub struct HeapItem {
    pub interval: (f64, f64),
    pub err: f64,
}

impl HeapItem {
    pub fn new(interval: (f64, f64), err: f64) -> Self {
        Self { interval, err }
    }
}

impl Eq for HeapItem {}

impl PartialEq for HeapItem {
    fn eq(&self, other: &Self) -> bool {
        self.err == other.err
    }
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.err).partial_cmp(&other.err).unwrap()
    }
}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
pub struct Myf64 {
    pub x: f64,
}
impl Myf64 {
    fn key(&self) -> u64 {
        self.x.to_bits()
    }
}

impl hash::Hash for Myf64 {
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        self.key().hash(state)
    }
}

impl PartialEq for Myf64 {
    fn eq(&self, other: &Myf64) -> bool {
        self.key() == other.key()
    }
}

impl Eq for Myf64 {}
