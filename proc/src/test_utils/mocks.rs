use crate::common::error_spanned;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

// 10/10 builder pattern

pub fn get_mock(name: Ident) -> TokenStream {
    match name.to_string().as_str() {
        "MockCollection" => {
            mock_collection()
        },
        _ => {
            error_spanned!("Unrecognized mock: {}", &name).to_compile_error()
        }
    }
}

fn mock_collection() -> TokenStream {
    quote!{
        use core::cell::RefCell;

        #[derive(Debug)]
        pub struct MockCollection {
            len: usize,
            length_calls: RefCell<usize>
        }

        impl Length for MockCollection {
            fn length(&self) -> usize {
                unsafe { *self.length_calls.as_ptr() += 1; }
                self.len
            }
        }

        impl core::ops::Index<usize> for MockCollection {
            type Output = usize;

            fn index(&self, _: usize) -> &Self::Output {
                &self.len
            }
        }

        impl core::ops::IndexMut<usize> for MockCollection {
            fn index_mut(&mut self, _: usize) -> &mut Self::Output {
                &mut self.len
            }
        }

        #[allow(dead_code)]
        impl MockCollection {
            pub fn new(len: usize) -> Self {
                Self { len, length_calls: RefCell::new(0) }
            }

            pub fn set_length(collection: *mut MockCollection, length: usize) {
                unsafe { *(&mut (*collection).len) = length }
            }

            pub fn length_calls(collection: *const MockCollection) -> usize {
                unsafe { *(*collection).length_calls.as_ptr() }
            }
        }
    }
}