//! # Deferred Vector Library
//!
//! `DeferredVec` is a Rust library providing a generic, lazily-initialized vector structure.
//! This structure is designed for scenarios where the initialization of a large or
//! resource-intensive vector should be deferred until it's actually needed.
//!
//! Developed by Prof. Afonso Miguel - PUCPR, this library is ideal for applications
//! where performance and efficient resource management are critical.
//!
//! ## Features
//!
//! - **Laziness**: The vector is not initialized until it's explicitly accessed. This
//!   lazy initialization is beneficial for performance in cases where the vector might not
//!   be used immediately or at all.
//! - **Flexibility**: Works with any type `T` that implements the `Clone` trait, allowing
//!   for a wide range of applications.
//! - **Custom Initialization**: The vector is initialized using a user-provided function,
//!   offering flexibility in how the vector's contents are determined.
//!
//! ## Usage
//!
//! To use `DeferredVec`, define a `fetch_function` that returns the initial state of the
//! vector when called. The `DeferredVec` struct can then be instantiated with this function.
//! The vector remains uninitialized (`None`) until methods like `get` or `len` are called,
//! at which point `fetch_function` is invoked to initialize the vector.
//!
//! ## Examples
//!
//! Basic usage:
//!
//! ```
//! let mut deferred_vector = DeferredVec::new(|| vec![1, 2, 3]);
//! assert_eq!(deferred_vector.is_deferred(), true);
//! let initialized_vector = deferred_vector.get();
//! assert_eq!(deferred_vector.is_deferred(), false);
//! ```
//!
//! ## Testing
//!
//! The module includes unit tests to verify the functionality, especially focusing on
//! the lazy initialization and basic vector operations.
//!
//! ## Disclaimer
//!
//! This code is provided as-is, without warranty. Users should thoroughly test it in their
//! own applications before any production use.
//!
//! ## Author
//!
//! Prof. Afonso Miguel - PUCPR - Original implementation and documentation.
//!
//! ## License
//!
//! This project is licensed under the MIT License - see the LICENSE file for details.


/// A generic struct `DeferredVec` for lazily initializing a vector.
///
/// This struct holds an `Option<Vec<T>>` to store the vector,
/// which may or may not be present initially, and a `fetch_function`
/// of type `fn() -> Vec<T>`, which is a function pointer
/// that returns a vector of the same type when called.
pub struct DeferredVec<T> {
    vec: Option<Vec<T>>,
    fetch_function: fn() -> Vec<T>,
}

/// Implement methods for `DeferredVec`.
///
/// The `#[allow(dead_code)]` attribute indicates
/// that even if some methods are not used, they should not be considered dead code.
/// The generic type `T` is bound by the trait `std::clone::Clone` to ensure
/// that elements of the vector can be cloned.
impl<T> DeferredVec<T>
where
    T: std::clone::Clone,
{
    /// Constructs a new instance of `DeferredVec`.
    ///
    /// # Arguments
    ///
    /// * `fetch_function` - A function to initialize the vector.
    ///
    /// # Returns
    ///
    /// A new instance of `DeferredVec` with `vec` initialized as `None`.
    pub fn new(fetch_function: fn() -> Vec<T>) -> DeferredVec<T> {
        DeferredVec {
            vec: None,
            fetch_function,
        }
    }

    /// Fetches and initializes the `vec` if it's `None`.
    ///
    /// Returns a cloned instance of the vector.
    ///
    /// # Returns
    ///
    /// An `Option<Vec<T>>` which is the cloned instance of the vector.
    fn fetch(&mut self) -> Option<Vec<T>> {
        if self.vec.is_none() {
            self.vec = Some((self.fetch_function)());
        }
        self.vec.clone()
    }

    /// Fetches and returns the vector.
    ///
    /// This method calls `fetch` and unwraps the result to get the vector.
    /// It panics if `fetch` returns `None`.
    ///
    /// # Returns
    ///
    /// The fetched vector.
    pub fn get(&mut self) -> Vec<T> {
        self.fetch().unwrap().clone()
    }

    /// Returns the length of the fetched vector.
    ///
    /// This method fetches the vector and returns its length.
    /// It panics if `fetch` returns `None`.
    ///
    /// # Returns
    ///
    /// The length of the fetched vector.
    pub fn len(&mut self) -> usize {
        if let Some(vec) = self.fetch() {
            return vec.len();
        }
        panic!("Should not happen");
    }

    /// Checks if the vector is initialized.
    ///
    /// # Returns
    ///
    /// `true` if `vec` is `None` (not yet fetched) and `false` otherwise.
    pub fn is_deferred(&self) -> bool {
        self.vec.is_none()
    }
}

/// Unit tests for `DeferredVec`.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tests the basic functionality of `DeferredVec`.
    ///
    /// This test ensures that the vector is initially deferred,
    /// and after calling `get`, it is no longer deferred and contains the correct values.
    fn it_works() {
        let mut tst = DeferredVec::new(|| vec![1, 2, 3]);
        assert_eq!(tst.is_deferred(), true);
        let v = tst.get();
        assert_eq!(tst.is_deferred(), false);
        assert_eq!(v, vec![1, 2, 3]);
    }
}
