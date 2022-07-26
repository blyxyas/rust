// compile-flags: -Zunleash-the-miri-inside-of-you
#![feature(core_intrinsics)]
#![allow(const_err)]

// During CTFE, we prevent pointer comparison and pointer-to-int casts.

static CMP: () = {
    let x = &0 as *const _;
    let _v = x == x;
    //~^ ERROR could not evaluate static initializer
    //~| "pointer arithmetic or comparison" needs an rfc before being allowed inside constants
};

static PTR_INT_CAST: () = {
    let x = &0 as *const _ as usize;
    //~^ ERROR could not evaluate static initializer
    //~| "exposing pointers" needs an rfc before being allowed inside constants
    let _v = x == x;
};

static PTR_INT_TRANSMUTE: () = unsafe {
    let x: usize = std::mem::transmute(&0);
    let _v = x + 0;
    //~^ ERROR could not evaluate static initializer
    //~| unable to turn pointer into raw bytes
};

fn main() {}
