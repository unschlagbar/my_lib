pub mod net;
pub mod io;
pub mod rand;
pub mod graphics;
pub mod ui;
pub mod primitives;
pub mod collections;


use std::time::Instant;

use collections::FixedVec;


fn main() {
    {
        let start_time = Instant::now();
    
        let depth = 100;
        let rows = 100;
        let cols = 100;
    
        let mut matrix = vec![vec![vec![0.0; cols]; rows]; depth];

        let init_time = start_time.elapsed();
        let start_time = Instant::now();
    
        for _ in 0..1_000 {
            for depth in 0..matrix.len() {
                for row in 0..matrix[0].len() {
                    for col in 0..matrix[0][0].len() {
                        matrix[depth][row][col] = row as f32 * matrix[depth][row][col];
                    }
                }
            }
        }
    
        println!("Vec<Vec<Vec>>: {:?}, {:?}", init_time, start_time.elapsed())
    }

    {
        let start_time = Instant::now();
    
        let depth = 100;
        let rows = 100;
        let cols = 100;
    
        let mut matrix = collections::Vec3D::new(depth, rows, cols);

        let init_time = start_time.elapsed();
        let start_time = Instant::now();
    
        for _ in 0..1_000 {
            for depth in 0..matrix.depth() {
                for row in 0..matrix.rows() {
                    for col in 0..matrix.cols() {
                        matrix[(depth, row, col)] = row as f32 * matrix[(depth, row, col)];
                    }
                }
            }
        }
    
        println!("Vec3D: {:?}, {:?}", init_time, start_time.elapsed())
    }

    {
        let start_time = Instant::now();

        let rows = 1000;
        let cols = 1000;

        let mut matrix = vec![vec![0.0; cols]; rows];

        let init_time = start_time.elapsed();
        let start_time = Instant::now();

        for _ in 0..1_000 {
            for row in 0..matrix.len() {
                for col in 0..matrix[0].len() {
                    matrix[row][col] = row as f32 * matrix[row][col];
                }
            }
        }

        println!("Vec<Vec>: {:?}, {:?}", init_time, start_time.elapsed())
    }

    {
        let start_time = Instant::now();
    
        let rows = 1000;
        let cols = 1000;
    
        let mut matrix = collections::Matrix::new(rows, cols);

        let init_time = start_time.elapsed();
        let start_time = Instant::now();
    
        for _ in 0..1_000 {
            for row in 0..matrix.rows() {
                for col in 0..matrix.cols() {
                    matrix[(row, col)] = row as f32 * matrix[(row, col)];
                }
            }
        }
    
        println!("Matrix: {:?}, {:?}", init_time, start_time.elapsed())
    }

    {
        let start_time = Instant::now();

        let len = 10000000;

        let mut vec = vec![5.0; len];

        let init_time = start_time.elapsed();
        let start_time = Instant::now();

        for _ in 0..10_0 {
            for i in 0..vec.len() {
                vec[i] = i as f32 * vec[i];
            }
        }

        println!("Vec: {:?}, {:?}", init_time, start_time.elapsed())
    }

    {
        let start_time = Instant::now();

        let len = 10000000;

        let mut vec = fixed_vec!(5.0; len);

        let init_time = start_time.elapsed();
        let start_time = Instant::now();

        for _ in 0..10_0 {
            for i in 0..len {
                vec[i] = i as f32 * vec[i];
            }
        }

        println!("FixedVec: {:?}, {:?}", init_time, start_time.elapsed())
    }
}