use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;
use std::ops::{Index, IndexMut};

pub struct FixedVec<T> {
    data: NonNull<T>, // Rohspeicher für die Daten
    len: usize,    // Feste Länge des Vektors
}

impl<T> FixedVec<T> {

    #[inline]
    pub fn new(len: usize) -> Self {
        debug_assert!(len > 0, "Len must not be 0");
        
        // Allokation des Speichers
        let layout = Layout::array::<T>(len).expect("Ungültiges Layout");
        let data = unsafe {
            let ptr = alloc(layout) as *mut T;
            if ptr.is_null() {
                panic!("Speicherallokation fehlgeschlagen");
            }
            NonNull::new_unchecked(ptr)
        };

        FixedVec { data, len }
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn set(&mut self, index: usize, value: T) {
        debug_assert!(index < self.len, "Index out of bounds");
        unsafe {
            *self.data.as_ptr().add(index) = value;
        }
    }

    #[inline]
    pub fn get(&self, index: usize) -> &T {
        debug_assert!(index < self.len, "Index out of bounds");
        unsafe {
            &*self.data.as_ptr().add(index)
        }
    }

    #[inline]
    pub fn get_mut(&mut self, index: usize) -> &mut T {
        debug_assert!(index < self.len, "Index out of bounds");
        unsafe {
            &mut *self.data.as_ptr().add(index)
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(self.data.as_ptr(), self.len)
        }
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut(self.data.as_ptr(), self.len)
        }
    }

    #[inline]
    ///slice should be no longer used
    pub unsafe fn from_slice(slice: &[T]) -> Self {
        let len = slice.len();
        debug_assert!(len > 0, "Slice must not be empty");


        FixedVec { data: unsafe { NonNull::new(slice.as_ptr() as *mut T).unwrap_unchecked() }, len }
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    #[inline]
    pub unsafe fn from_raw_parts(ptr: *mut T, len: usize) -> Self {
        debug_assert!(len > 0, "Len must not be 0");

        let data = NonNull::new(ptr).expect("Pointer must not be null");

        FixedVec { data, len }
    }
}

impl<T: Copy> FixedVec<T> {
    // Allokiert den Speicher und initialisiert den Vektor mit einem Wert
    pub fn with_value(len: usize, value: T) -> Self {
        debug_assert!(len > 0, "FixedVec muss eine nicht-leere Länge haben");

        let layout = Layout::array::<T>(len).expect("Ungültiges Layout");
        let data = unsafe {
            let ptr = alloc(layout) as *mut T;
            if ptr.is_null() {
                panic!("Speicherallokation fehlgeschlagen");
            }

            // Initialisieren der Werte mit dem angegebenen Standardwert
            for i in 0..len {
                ptr.add(i).write(value); // Schreibe 'value' an alle Positionen
            }

            NonNull::new_unchecked(ptr)
        };

        FixedVec { data, len }
    }

}

impl<T: Copy> From<&[T]> for FixedVec<T> {
    fn from(slice: &[T]) -> Self {
        unsafe { FixedVec::from_slice(slice) }
    }
}

// Implementierung für `From<&[T; N]>` für statische Arrays
impl<T: Copy, const N: usize> From<&[T; N]> for FixedVec<T> {
    fn from(array: &[T; N]) -> Self {
        unsafe { FixedVec::from_slice(array) }
    }
}

impl<T> AsRef<[T]> for FixedVec<T> {
    fn as_ref(&self) -> &[T] {
        self.as_slice() // Nutzt die bereits implementierte Methode
    }
}

impl<T> AsMut<[T]> for FixedVec<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice() // Nutzt die bereits implementierte Methode
    }
}


// Ermöglicht den Zugriff über Index-Operatoren
impl<T> Index<usize> for FixedVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
    }
}

impl<T> IndexMut<usize> for FixedVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index)
    }
}

impl<T> Clone for FixedVec<T> {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl<T> Drop for FixedVec<T> {
    fn drop(&mut self) {
        let layout = unsafe { Layout::array::<T>(self.len).unwrap_unchecked() };
        unsafe {
            dealloc(self.data.as_ptr() as *mut u8, layout);
        }
    }
}

#[macro_export]
macro_rules! fixed_vec {
    // Regel für die Notation `fixed_vec![value; len]`
    ($x:expr; $len:expr) => {{
        let len = $len;
        let value = $x;
        FixedVec::with_value(len, value)
    }};

    // Regel für die Notation `fixed_vec![v1, v2, v3, ...]`
    ($($x:expr),+ $(,)?) => {{
        let slice = &[$($x),+];
        FixedVec::from(slice)
    }};
}

