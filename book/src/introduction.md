# Introduction

PJDFSTest is a file system test suite.
It was originally written to validate the ZFS port to FreeBSD,
but it now supports multiple operating systems and file systems.
This is a complete rewrite of the original test suite in Rust.

## Build

```sh
cd rust
cargo run
```

## Architecture

The package is made of:

- a library, which contains all the tests,
- a binary, which is the test runner.

### Library

The library exports modules, which contains groups of test cases, generally grouped by syscall.
A test case is itself composed of multiple test functions. 

### Binary

The binary is the test runner.
