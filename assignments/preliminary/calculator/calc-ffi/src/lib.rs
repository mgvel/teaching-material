use calc::prelude::*;
use std::ffi::CStr;
use std::os::raw::c_char;

// What I want to expose:
// * single function
// * takes a C string of "input"
// * returns (if successful) the output

// parse_and_eval
//
// Arguments?
//    * const char
// Return type?
//    * integer

// returns 0 if success, returns nonzero if failure
// on success, output is updated with the result
//
// int parse_and_eval(char*, int64_t* output);

pub use calc::Expr;

#[no_mangle]
pub extern "C" fn parse_and_eval(maybe_cstr: *const c_char, output: *mut i64) -> isize {
    // Validate input parameters
    todo!();

    // execute parse & eval
    todo!();

    // if successful, set output. Mind the ownership!
    todo!();
}

/// This will return null if unsuccessful
#[no_mangle]
pub extern "C" fn c_parse(maybe_cstr: *const c_char) -> *mut Expr {
    // validate input, return null pointer if invalid
    todo!();

    // return a raw pointer to an object whose ownership
    // has been given up on purpose (a "leak")
    // so that it can be freely used in the FFI caller
    // who later has to call our release function
    todo!();
}

#[no_mangle]
pub extern "C" fn c_eval(expr: *const Expr, output: *mut i64) -> isize {
    // validate `expr` and `output`
    todo!();

    // execute eval() and set output if successful
    todo!();
}

#[no_mangle]
/// release objects whose ownership was previously passed to C
pub extern "C" fn release_expr(box_expr: *mut Expr) {
    // take ownership back and go out of scope to drop
    todo!();
}
