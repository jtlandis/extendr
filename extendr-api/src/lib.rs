//!
//! extendr - A safe and user friendly R extension interface.
//!
//! This library aims to provide an interface that will be familiar to
//! first-time users of Rust or indeed any compiled language.
//!
//! Anyone who knows the R library should be able to write R extensions.
//! 
//! See the [Robj](../robj/enum.Robj.html) struct for much of the content of this crate.
//! [Robj](../robj/enum.Robj.html) provides a safe wrapper for the R object type.
//! 
//! This library is just being born, but goals are:
//!
//! Implement common R functions such as c() and print()
//!
//! Example:
//!
//! ```ignore
//! let v = c!(1, 2, 3);
//! let l = list!(a=1, b=2);
//! print!(v, l);
//! ```
//!
//! Provide a wrapper for r objects.
//!
//! Example:
//!
//! ```ignore
//! let s = Robj::from("hello");
//! let i = Robj::from(1);
//! let r = Robj::from(1.0);
//! ```
//!
//! Provide iterator support for creation and consumption of r vectors.
//!
//! Example:
//!
//! ```ignore
//! let res = (1..=100).iter().collect::<Robj>();
//! for x in res {
//!     print!(x);
//! }
//! ```
//!
//! Provide a procedural macro to adapt Rust functions to R
//!
//! Example:
//!
//! ```ignore
//! #[extendr]
//! fn fred(a: i32) -> i32 {
//!     a + 1
//! }
//! ```
//!
//! In R:
//!
//! ```ignore
//!
//! result <- .Call("fred", 1)
//!
//! ```
//!

mod args;
mod engine;
mod logical;
mod rmacros;
mod robj;
mod wrapper;

pub use args::*;
pub use engine::*;
pub use rmacros::*;
pub use robj::*;
pub use wrapper::*;

pub use extendr_macros::*;
pub use libR_sys::DllInfo;
pub use libR_sys::R_CallMethodDef;
pub use libR_sys::R_forceSymbols;
pub use libR_sys::R_registerRoutines;
pub use libR_sys::R_useDynamicSymbols;
pub use libR_sys::SEXP;

/// Generic dynamic error type.
pub type AnyError = Box<dyn std::error::Error + Send + Sync>;

pub struct CallMethod {
    pub call_symbol: std::ffi::CString,
    pub func_ptr: *const u8,
    pub num_args: i32,
}

// Internal function used to implement the .Call interface.
// This is called from the code generated by the #[extendr] attribute.
pub unsafe fn register_call_methods(info: *mut libR_sys::DllInfo, methods: &[CallMethod]) {
    let mut rmethods: Vec<_> = methods
        .iter()
        .map(|m| libR_sys::R_CallMethodDef {
            name: m.call_symbol.as_ptr(),
            fun: Some(std::mem::transmute(m.func_ptr)),
            numArgs: m.num_args,
        })
        .collect();

    rmethods.push(libR_sys::R_CallMethodDef {
        name: std::ptr::null(),
        fun: None,
        numArgs: 0,
    });

    libR_sys::R_registerRoutines(
        info,
        std::ptr::null(),
        rmethods.as_ptr(),
        std::ptr::null(),
        std::ptr::null(),
    );
    //libR_sys::R_useDynamicSymbols(info, 0);
    //libR_sys::R_forceSymbols(info, 1);
}

// pub fn add_function_to_namespace(namespace: &str, fn_name: &str, wrap_name: &str) {
//     let rcode = format!("{}::{} <- function(...) .Call(\"{}\", ...)", namespace, fn_name, wrap_name);
//     eprintln!("[{}]", rcode);
//     Robj::eval_string(rcode.as_str()).unwrap();
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate as extendr_api;
    use extendr_macros::extendr;
    use extendr_macros::extendr_module;
    #[extendr]
    pub fn inttypes(a: i8, b: u8, c: i16, d: u16, e: i32, f: u32, g: i64, h: u64) {
        assert_eq!(a, 1);
        assert_eq!(b, 2);
        assert_eq!(c, 3);
        assert_eq!(d, 4);
        assert_eq!(e, 5);
        assert_eq!(f, 6);
        assert_eq!(g, 7);
        assert_eq!(h, 8);
    }

    #[extendr]
    pub fn floattypes(a: f32, b: f64) {
        assert_eq!(a, 1.);
        assert_eq!(b, 2.);
    }

    #[extendr]
    pub fn strtypes(a: &str, b: String) {
        assert_eq!(a, "abc");
        assert_eq!(b, "def");
    }

    #[extendr]
    pub fn vectortypes(a: Vec<i32>, b: Vec<f64>) {
        assert_eq!(a, [1, 2, 3]);
        assert_eq!(b, [4., 5., 6.]);
    }

    #[extendr]
    pub fn robjtype(a: Robj) {
        assert_eq!(a, Robj::from(1))
    }

    #[extendr]
    pub fn return_u8() -> u8 {
        123
    }

    #[extendr]
    pub fn return_u16() -> u16 {
        123
    }

    #[extendr]
    pub fn return_u32() -> u32 {
        123
    }

    #[extendr]
    pub fn return_u64() -> u64 {
        123
    }

    #[extendr]
    pub fn return_i8() -> i8 {
        123
    }

    #[extendr]
    pub fn return_i16() -> i16 {
        123
    }

    #[extendr]
    pub fn return_i32() -> i32 {
        123
    }

    #[extendr]
    pub fn return_i64() -> i64 {
        123
    }

    #[extendr]
    pub fn return_f32() -> f32 {
        123.
    }

    #[extendr]
    pub fn return_f64() -> f64 {
        123.
    }

    struct Person {
        pub name: String,
    }

    #[extendr]
    impl Person {
        fn new() -> Self {
            Self {
                name: "".to_string(),
            }
        }

        fn set_name(&mut self, name: &str) {
            self.name = name.to_string();
        }

        fn name(&self) -> &str {
            self.name.as_str()
        }
    }

    #[extendr]
    fn aux_func(_person: &Person) {}

    // Macro to generate exports
    extendr_module! {
        mod my_module;
        fn aux_func;
        impl Person;
    }

    #[test]
    fn export_test() {
        use super::*;
        // Call the exported functions through their generated C wrappers.
        unsafe {
            wrap__inttypes(
                Robj::from(1).get(),
                Robj::from(2).get(),
                Robj::from(3).get(),
                Robj::from(4).get(),
                Robj::from(5).get(),
                Robj::from(6).get(),
                Robj::from(7).get(),
                Robj::from(8).get(),
            );
            wrap__inttypes(
                Robj::from(1.).get(),
                Robj::from(2.).get(),
                Robj::from(3.).get(),
                Robj::from(4.).get(),
                Robj::from(5.).get(),
                Robj::from(6.).get(),
                Robj::from(7.).get(),
                Robj::from(8.).get(),
            );
            wrap__floattypes(Robj::from(1.).get(), Robj::from(2.).get());
            wrap__floattypes(Robj::from(1).get(), Robj::from(2).get());
            wrap__strtypes(Robj::from("abc").get(), Robj::from("def").get());
            wrap__vectortypes(
                Robj::from(&[1, 2, 3] as &[i32]).get(),
                Robj::from(&[4., 5., 6.] as &[f64]).get(),
            );
            wrap__robjtype(Robj::from(1).get());
            assert_eq!(new_borrowed(wrap__return_u8()), Robj::from(123));
            assert_eq!(new_borrowed(wrap__return_u16()), Robj::from(123));
            assert_eq!(new_borrowed(wrap__return_u32()), Robj::from(123));
            assert_eq!(new_borrowed(wrap__return_u64()), Robj::from(123));
            assert_eq!(new_borrowed(wrap__return_i8()), Robj::from(123));
            assert_eq!(new_borrowed(wrap__return_i16()), Robj::from(123));
            assert_eq!(new_borrowed(wrap__return_i32()), Robj::from(123));
            assert_eq!(new_borrowed(wrap__return_i64()), Robj::from(123));
            assert_eq!(new_borrowed(wrap__return_f32()), Robj::from(123.));
            assert_eq!(new_borrowed(wrap__return_f64()), Robj::from(123.));
        }
    }
}
