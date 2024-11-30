#![allow(dead_code)]

use std::{alloc::{alloc, dealloc, Layout}, ops::{Index, IndexMut}, ptr::NonNull};

pub struct Matrix {
    data: NonNull<f32>, // Rohspeicher für die Matrixdaten
    rows: usize,
    cols: usize,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        let size = rows * cols;
        let layout = unsafe { Layout::array::<f32>(size).unwrap_unchecked() };

        let data = unsafe {
            let ptr = alloc(layout) as *mut f32;

            #[cfg(debug_assertions)]
            if ptr.is_null() {
                panic!("Speicherallokation fehlgeschlagen");
            }
            NonNull::new_unchecked(ptr)
        };

        Matrix { data, rows, cols }
    }

    pub const fn rows(&self) -> usize {
        self.rows
    }

    pub const fn cols(&self) -> usize {
        self.cols
    }

    pub const fn flat_len(&self) -> usize {
        self.rows * self.cols
    }

    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        debug_assert!(row < self.rows && col < self.cols);

        let index = row * self.cols + col;
        unsafe { *self.data.as_ptr().add(index) = value };
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        debug_assert!(row < self.rows && col < self.cols);

        let index = row * self.cols + col;
        unsafe { *self.data.as_ptr().add(index) }
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        debug_assert!(row < self.rows && col < self.cols);

        let idx = row * self.cols + col;
        unsafe { &*self.data.as_ptr().add(idx) }
    }
}

// Implementierung von IndexMut, um das Schreiben über [] zu ermöglichen
impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, col) = index;
        debug_assert!(row < self.rows && col < self.cols);

        let idx = row * self.cols + col;
        unsafe { &mut *self.data.as_ptr().add(idx) }
    }
}

impl Index<usize> for Matrix {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(self.rows * self.cols > index);

        unsafe { &*self.data.as_ptr().add(index) }
    }
}

// Implementierung von IndexMut, um das Schreiben über [] zu ermöglichen
impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(self.rows * self.cols > index);

        unsafe { &mut *self.data.as_ptr().add(index) }
    }
}

impl Drop for Matrix {
    fn drop(&mut self) {
        let size = self.rows * self.cols;
        let layout = unsafe { Layout::array::<f32>(size).unwrap_unchecked() };
        unsafe {
            dealloc(self.data.as_ptr() as _, layout);
        }
    }
}