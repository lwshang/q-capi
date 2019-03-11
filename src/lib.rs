//  https://code.kx.com/q/interfaces/c-client-for-q/

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw;

pub type H = raw::c_short;
pub type I = raw::c_int;
pub type J = raw::c_longlong;
pub type S = *mut raw::c_char;
pub type Sconst = *const raw::c_char;
pub type G = raw::c_uchar;
pub type C = raw::c_char;
pub type E = raw::c_float;
pub type F = raw::c_double;
pub type V = raw::c_void;
pub type callback_func = unsafe extern "C" fn(I) -> K; // used in sd1

#[repr(C)]
#[derive(Copy, Clone)]
pub struct k0 {
    pub m: raw::c_schar, // m,a are for internal use.
    pub a: raw::c_schar,
    pub t: raw::c_schar, // The object's type
    pub u: C,            // The object's attribute flags
    pub r: I,            // The object's reference count
    pub union_in_k0: Union_in_k0,
}

pub type K = *mut k0;
pub type Kconst = *const k0;

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
    pub n: J, // number of elements in vector
    pub G0: [G; 1usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct U {
    pub g: [G; 16usize],
}

// https://code.kx.com/v2/interfaces/capiref/
extern "C" {
    pub fn b9(arg1: I, arg2: K) -> K;
    pub fn d9(arg1: K) -> K; // deserialize
    pub fn dj(arg1: I) -> I;
    pub fn dl(arg1: *mut V, arg2: I) -> K;
    pub fn dot(arg1: K, arg2: K) -> K;
    pub fn ee(arg1: K) -> K;
    pub fn ja(arg1: *mut K, arg2: *mut V) -> K;
    pub fn jk(arg1: *mut K, arg2: K) -> K;
    pub fn js(arg1: *mut K, arg2: S) -> K;
    pub fn jv(k: *mut K, arg2: K) -> K;
    pub fn k(arg1: I, arg2: Sconst, ...) -> K;
    pub fn ka(arg1: I) -> K;
    pub fn kb(arg1: I) -> K;
    pub fn kc(arg1: I) -> K;
    pub fn kclose(arg1: I) -> V;
    pub fn kd(arg1: I) -> K;
    pub fn ke(arg1: F) -> K;
    pub fn kf(arg1: F) -> K;
    pub fn kg(arg1: I) -> K;
    pub fn kh(arg1: I) -> K;
    pub fn khp(arg1: Sconst, arg2: I) -> I;
    pub fn khpu(arg1: Sconst, arg2: I, arg3: Sconst) -> I;
    pub fn khpun(arg1: Sconst, arg2: I, arg3: Sconst, arg4: I) -> I;
    pub fn khpunc(arg1: Sconst, arg2: I, arg3: Sconst, arg4: I, arg5: I) -> I;
    pub fn ki(arg1: I) -> K;
    pub fn kj(arg1: J) -> K;
    pub fn knk(arg1: I, ...) -> K;
    pub fn knt(arg1: J, arg2: K) -> K;
    pub fn kp(arg1: S) -> K;
    pub fn kpn(arg1: S, arg2: J) -> K;
    pub fn krr(arg1: Sconst) -> K;
    pub fn ks(arg1: S) -> K;
    pub fn kt(arg1: I) -> K;
    pub fn ktd(arg1: K) -> K;
    pub fn ktj(arg1: I, arg2: J) -> K; // ?
    pub fn ktn(arg1: I, arg2: J) -> K;
    pub fn ku(arg1: U) -> K;
    pub fn kz(arg1: F) -> K;
    pub fn m9(arg1: V) -> V;
    pub fn okx(arg1: K) -> I;
    pub fn orr(arg1: Sconst) -> K;
    pub fn r0(arg1: K) -> V;
    pub fn r1(arg1: K) -> K;
    pub fn sd0(arg1: I) -> V;
    pub fn sd0x(arg1: I, arg2: I) -> V;
    pub fn sd1(arg1: I, arg2: callback_func) -> K;
    //setm
    pub fn sn(arg1: S, arg2: I) -> S;
    pub fn ss(arg1: S) -> S;
    pub fn sslInfo(arg1: K) -> K;
    //ver
    pub fn xD(arg1: K, arg2: K) -> K;
    pub fn xT(arg1: K) -> K;
    pub fn ymd(arg1: I, arg2: I, arg3: I) -> I;
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::ffi::CString;
    use std::ptr;

    #[test]
    fn size_of_k0() {
        assert_eq!(std::mem::size_of::<k0>(), 24);
    }

    #[test]
    fn test_api() {
        unsafe {
            let hostname = CString::new("localhost").unwrap();
            let h = khp(hostname.as_ptr(), 5000);
            let query = CString::new("a:3").unwrap();
            let _ = k(h, query.as_ptr(), ptr::null() as *const V);
            kclose(h);
        }
    }
}
