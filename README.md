# Rust binding for q/kdb+ [WIP]

[![Build Status](https://travis-ci.org/lwshang/q-capi.svg?branch=master)](https://travis-ci.org/lwshang/q-capi)

This crate provides low-level binding (FFI) of the C API to q/kdb+. A high-level idiomatic Rust interface will be developed base on it.

It can be used to accomplish all kinds of missions the original C API can do:

* Create a client to communicate with kdb+ (requiring linked with platform dependent pre-compiled objects)
* Build dynamically-loaded extensions to extend q functionality

Currently, this project is not mature enough to be published to crates.io. But this doesn't mean that it's not worth trying. Because the development is committed to be consistant with the official [documents](https://code.kx.com). The function bindings will have the same signatures as original C API (enforced by `ctest`).

## Requirement

* macOS/linux on x86_64 platform
* C compiling environment
  * macOS: Xcode / commandline tools 
  * linux: gcc

## Known issue

Because of the language design choice in Rust, in some rare cases, it is not possible to provide exactly the same interfaces as C.

In this project, the orginal C API defined structs with anonymous struct/union which is not allowed in Rust. We have to name those anonymous ones so that we can access the members. This is an inevitable inconsistence. In plan, the high-level Rust interface will provide some idiomatic functions which hide the underling definition of the struct.

## TODO

- [ ] test opaque struct U
- [ ] vak vaknk
- [ ] test functionality
- [ ] examples
- [ ] more OS or platforms


## License

This project is open source with MIT License.

The object files (i.e. `e.o`) are copied from [kdb](https://github.com/KxSystems/kdb) which is open source with Apache License 2.0.

**Notice**

> kdb+ is a proprietary database developed by Kx Systems. It has a built-in programming language q. Users can get the compiled binary executables (`q`) from Kx and using it under some License.

This project is not naming as `libkdb-sys` because kdb is not a system library which can be installed by some package manager on any OS. 