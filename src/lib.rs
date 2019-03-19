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
// pub type callback_func = unsafe extern "C" fn(I) -> K; // used in sd1

#[repr(C)]
#[derive(Copy, Clone)]
pub struct k0 {
    pub m: raw::c_schar, // m,a are for internal use.
    pub a: raw::c_schar,
    pub t: raw::c_schar, // The object's type
    pub u: C,            // The object's attribute flags
    pub r: I,            // The object's reference count
    _value: ValueUnion,
}

pub type K = *mut k0;
// pub type Kconst = *const k0;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union ValueUnion {
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
    _collection: Collection,
    // align as 128 bits
    _union_align: [u64; 2usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Collection {
    pub n: J,            // number of elements in vector
    pub G0: [G; 1usize], // byte kG / char kC / pointer to underlying vectors
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct U {
    pub g: [G; 16usize],
}

// https://code.kx.com/v2/interfaces/capiref/
extern "C" {
    /// serialize
    pub fn b9(arg1: I, arg2: K) -> K;
    /// deserialize
    pub fn d9(arg1: K) -> K;
    /// date to number
    pub fn dj(arg1: I) -> I;
    /// dynamic link
    // pub fn dl(arg1: *mut V, arg2: I) -> K;
    /// apply
    // pub fn dot(arg1: K, arg2: K) -> K;
    /// error string
    pub fn ee(arg1: K) -> K;
    /// join value
    pub fn ja(arg1: *mut K, arg2: *mut V) -> K;
    /// join K object
    pub fn jk(arg1: *mut K, arg2: K) -> K;
    /// join string
    pub fn js(arg1: *mut K, arg2: S) -> K;
    /// join K lists
    pub fn jv(k: *mut K, arg2: K) -> K;
    /// evaluate
    pub fn k(arg1: I, arg2: Sconst, ...) -> K;
    /// create atom
    pub fn ka(arg1: I) -> K;
    /// create boolean
    pub fn kb(arg1: I) -> K;
    /// create char
    pub fn kc(arg1: I) -> K;
    /// disconnect
    pub fn kclose(arg1: I) -> V;
    /// create date
    pub fn kd(arg1: I) -> K;
    /// create real
    pub fn ke(arg1: F) -> K;
    /// create float
    pub fn kf(arg1: F) -> K;
    /// create byte
    pub fn kg(arg1: I) -> K;
    /// create short
    pub fn kh(arg1: I) -> K;
    /// connect anonymously
    pub fn khp(arg1: Sconst, arg2: I) -> I;
    /// connect, no timeout
    pub fn khpu(arg1: Sconst, arg2: I, arg3: Sconst) -> I;
    /// connect
    pub fn khpun(arg1: Sconst, arg2: I, arg3: Sconst, arg4: I) -> I;
    /// connect with capability
    pub fn khpunc(arg1: Sconst, arg2: I, arg3: Sconst, arg4: I, arg5: I) -> I;
    /// creat int
    pub fn ki(arg1: I) -> K;
    /// create  long
    pub fn kj(arg1: J) -> K;
    /// create list
    pub fn knk(arg1: I, ...) -> K;
    /// create keyed table
    pub fn knt(arg1: J, arg2: K) -> K;
    /// create string
    pub fn kp(arg1: S) -> K;
    /// create fixed-length string
    pub fn kpn(arg1: S, arg2: J) -> K;
    /// signal C error
    pub fn krr(arg1: Sconst) -> K;
    /// create symbol
    pub fn ks(arg1: S) -> K;
    /// create time
    pub fn kt(arg1: I) -> K;
    /// create simple table
    pub fn ktd(arg1: K) -> K;
    /// create timestamp / create timespan
    pub fn ktj(arg1: I, arg2: J) -> K;
    /// create vector
    pub fn ktn(arg1: I, arg2: J) -> K;
    /// create guid
    pub fn ku(arg1: U) -> K;
    /// create datetime
    pub fn kz(arg1: F) -> K;
    /// release memory
    pub fn m9(arg1: V) -> V;
    /// verify IPC message
    pub fn okx(arg1: K) -> I;
    /// signal system error¶
    pub fn orr(arg1: Sconst) -> K;
    /// decrement refcount
    pub fn r0(arg1: K) -> V;
    /// increment refcount
    pub fn r1(arg1: K) -> K;
    /// remove callback
    // pub fn sd0(arg1: I) -> V;
    /// remove callback conditional
    // pub fn sd0x(arg1: I, arg2: I) -> V;
    /// set function on loop
    // pub fn sd1(arg1: I, arg2: extern "C" fn(I) -> K) -> K;
    // toggle symbol lock
    pub fn setm(arg1: I) -> I;
    /// intern chars
    pub fn sn(arg1: S, arg2: I) -> S;
    /// intern string
    pub fn ss(arg1: S) -> S;
    /// sslInfo
    pub fn sslInfo(arg1: K) -> K;
    /// release date
    pub fn ver() -> I;
    /// create dictionary
    pub fn xD(arg1: K, arg2: K) -> K;
    /// table from dictioinary
    pub fn xT(arg1: K) -> K;
    /// numbers to date
    pub fn ymd(arg1: I, arg2: I, arg3: I) -> I;
}

// #[cfg(test)]
// mod tests {
//     use crate::*;
//     use std::ffi::CString;
//     use std::ptr;

//     #[test]
//     fn size_of_k0() {
//         assert_eq!(std::mem::size_of::<k0>(), 24);
//     }

//     #[test]
//     fn test_api() {
//         unsafe {
//             let hostname = CString::new("localhost").unwrap();
//             let h = khp(hostname.as_ptr(), 5000);
//             if h > 0 {
//                 let query = CString::new("a:3").unwrap();
//                 let _ = k(h, query.as_ptr(), ptr::null() as *const V);
//                 let query = CString::new("a").unwrap();
//                 let res = k(h, query.as_ptr(), ptr::null() as *const V);
//                 kclose(h);
//                 assert_eq!((*res)._value.j, 3);
//             }
//         }
//     }
// }
