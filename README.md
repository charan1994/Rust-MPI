# MPI support in Rust
This repository contains basic code and some runtimes to test how well MPI is supported in Rust.

The repository contains basic examples of a send-receive and an all-to-all communication pattern to send a fixed size of message between the processes. It also contains barriers used to time the program.

The development of the code was done using the latest version of Intel oneapi HPC kit(version 2021.1.0) and final deployment on the UB CCR cluster with intel-mpi 2020.1. The different mount and related parts to achieve this can be found as comments in the singularity definiton file. The `%files` and `%environment` have to be updated accordingly based on the individual's development and final runtime environment.

The given examples skip the default-features of rsmpi which contains `user-operations` and `derive` as I don't use these features. You can read more about these optional features on the rsmpi github readme.

## Requirements
1. Intel MPI compiler and related tools (either oneapi or intel-mpi based installs).
2. Libfabric.
3. Rust.
4. Clang compiler for some rust crates which are compiled from source.

## Resources
- [MPI standard introduction](https://en.wikipedia.org/wiki/Message_Passing_Interface)
- [Rust basics and installation resources](https://www.rust-lang.org/learn/get-started)
- [Singularity resources](https://singularity.lbl.gov/quickstart)
- [Rust mpi(rsmpi) github page](https://github.com/rsmpi/rsmpi)
- [rsmpi documentation](http://rsmpi.github.io/rsmpi/mpi/index.html)
- [more rsmpi examples](https://github.com/rsmpi/rsmpi/tree/master/examples)
- [one api installation link](https://software.intel.com/content/www/us/en/develop/tools/oneapi/base-toolkit/download.html)
