//  https://code.kx.com/q/interfaces/c-client-for-q/

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw;

pub type H = raw::c_short;
pub type I = raw::c_int;
pub type J = raw::c_longlong;
pub type S = *mut raw::c_char;
pub type G = raw::c_uchar;
pub type C = raw::c_char;
pub type E = raw::c_float;
pub type F = raw::c_double;
pub type V = raw::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct k0 {
    pub m: raw::c_schar,     // m,a are for internal use.
    pub a: raw::c_schar,
    pub t: raw::c_schar,     // The object's type
    pub u: C,                // The object's attribute flags
    pub r: I,                // The object's reference count
    pub union_in_k0: Union_in_k0,
}

pub type K = *mut k0;

#[repr(C)]
#[derive(Copy, Clone)]
pub union Union_in_k0 {
    // The atoms are held in the following members:
    pub g: G,
    pub h: H,
    pub i: I,
    pub j: J,
    pub e: E,
    pub f: F,
    pub s: S,
    // The following members are used for more complex data.
    pub k: *mut k0,
    pub struct_in_union: Struct_in_union,
    // _bindgen_union_align: [u64; 2usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_in_union {
    pub n: J,                // number of elements in vector
    pub G0: [G; 1usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct U {
    pub g: [G; 16usize],
}

#[cfg(test)]
mod tests {
    use crate::k0;
    #[test]
    fn size_of_k0() {
        assert_eq!(std::mem::size_of::<k0>(), 24);
    }
}