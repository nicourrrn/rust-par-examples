use rand::Rng;
use rayon::prelude::*;


use std::time;

type Matrix = Vec<Vec<i32>>;

fn sync_matrix_sum(m1: &Matrix, m2: &Matrix) -> Matrix {
    (0..(m1.len()))
        .map(|i| (0..m1[i].len()).map(|j| m1[i][j] + m2[i][j]).collect())
        .collect()
}

fn rayon_matrix_sum(m1: &Matrix, m2: &Matrix) -> Matrix {
    (0..(m1.len()))
        .into_par_iter()
        .map(|i| (0..m1[i].len()).map(|j| m1[i][j] + m2[i][j]).collect())
        .collect()
}

fn main() {
    let m1 = (0..1000)
        .map(|_| {
            (0..1000)
                .map(|_| rand::thread_rng().gen_range(0..100))
                .collect()
        })
        .collect();
    let m2 = (0..1000)
        .map(|_| {
            (0..1000)
                .map(|_| rand::thread_rng().gen_range(0..100))
                .collect()
        })
        .collect();

    let mut start = time::Instant::now();
    sync_matrix_sum(&m1, &m2);
    println!("Sync  with time {}", start.elapsed().as_nanos());
    start = time::Instant::now();
    rayon_matrix_sum(&m1, &m2);
    println!("Rayon with time {}", start.elapsed().as_nanos());
}
