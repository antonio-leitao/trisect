use crate::linalg;
use anyhow::{anyhow, Result};
use hashbrown::HashSet;
use rand::Rng;
use rayon::prelude::*;
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Neighbours {
    first: usize,
    second: usize,
}

impl Neighbours {
    pub fn new(a: usize, b: usize) -> Neighbours {
        // Ensure the smaller value is stored in `first`
        if a < b {
            Neighbours {
                first: a,
                second: b,
            }
        } else {
            Neighbours {
                first: b,
                second: a,
            }
        }
    }
}

fn dataset_dimensions_and_extremes(dataset: &[Vec<f64>]) -> Result<(usize, Vec<f64>, Vec<f64>)> {
    let mut min_values: Vec<f64>;
    let mut max_values: Vec<f64>;
    let dimensions = match dataset.first() {
        Some(first_point) => {
            max_values = first_point.clone();
            min_values = first_point.clone();
            first_point.len()
        }
        None => return Err(anyhow!("Dataset is empty")),
    };

    for point in dataset.iter().skip(1) {
        for (dim, &value) in point.iter().enumerate() {
            min_values[dim] = min_values[dim].min(value);
            max_values[dim] = max_values[dim].max(value);
        }
    }

    Ok((dimensions, min_values, max_values))
}

fn sample_point(dimensions: usize, min_values: &[f64], max_values: &[f64]) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let coords: Vec<f64> = (0..dimensions)
        .map(|dim| rng.gen_range(min_values[dim]..max_values[dim]))
        .collect();
    coords
}

fn nearest_neighbours(point: &[f64], points: &[Vec<f64>]) -> Neighbours {
    assert!(!points.is_empty(), "Vector of points must not be empty.");

    let mut distances = Vec::with_capacity(points.len());
    for (i, p) in points.iter().enumerate() {
        let distance = linalg::euclidean(point, p);
        distances.push((i, distance));
    }

    distances.sort_by(|(_, dist1), (_, dist2)| dist1.partial_cmp(dist2).unwrap_or(Ordering::Equal));

    // Get the indices of the two nearest neighbors
    let (u, _) = distances[0];
    let (v, _) = distances[1];

    Neighbours::new(u, v)
}

pub fn core_loop(data: Vec<Vec<f64>>, n_points: usize) -> Vec<(usize, usize)> {
    // Get  the max and min values
    let (dimensions, min_values, max_values) = dataset_dimensions_and_extremes(&data).unwrap();
    let edges: HashSet<Neighbours> = (0..n_points)
        .map(|_| {
            //generate a sample point
            let point = sample_point(dimensions, &min_values, &max_values);
            //find nearest neighbour
            nearest_neighbours(&point, &data)
        })
        .collect();
    edges
        .into_iter()
        .map(|neigh| (neigh.first, neigh.second))
        .collect()
}

pub fn core_loop_parallel(data: Vec<Vec<f64>>, n_points: usize) -> Vec<(usize, usize)> {
    // Get  the max and min values
    let (dimensions, min_values, max_values) = dataset_dimensions_and_extremes(&data).unwrap();
    let edges: HashSet<Neighbours> = (0..n_points)
        .into_par_iter()
        .map(|_| {
            //generate a sample point
            let point = sample_point(dimensions, &min_values, &max_values);
            //find nearest neighbour
            nearest_neighbours(&point, &data)
        })
        .collect();

    edges
        .into_iter()
        .map(|neigh| (neigh.first, neigh.second))
        .collect()
}
