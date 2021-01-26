# MPI support in Rust
This repository contains basic code and some runtimes to test how well MPI is supported in Rust.

The repository contains basic examples of a send-receive and an all-to-all communication pattern to send a fixed size of message between the processes. It also contains barriers used to time the program.

The development of the code was done using the latest version of Intel oneapi HPC kit(version 2021.1.0) and final deployment on the UB CCR cluster with intel-mpi 2020.1. The different mount and related parts to achieve this can be found as comments in the singularity definiton file. The `%files` and `%environment` have to be updated accordingly based on the individual's development and final runtime environment.

The given examples skip the default-features of rsmpi which contains `user-operations` and `derive` as I don't use these features. You can read more about these optional features on the rsmpi github readme.

## Contents of repository
The repository contains the following things:
- Code for all-to-all communication in Rust and C++.
- Code for point to point, send receive communication in the form of circle pass in Rust and C++.
- Logs of slurm scripts run at different configuration.
- Documents and jupyter notebook which compare the Rust and C++ runtimes, storage and other stats.

## Requirements
1. Intel MPI compiler and related tools (either oneapi or intel-mpi based installs).
2. Libfabric.
3. Rust.
4. Clang compiler for some rust crates which are compiled from source.

## Resources
- [MPI standard introduction](https://en.wikipedia.org/wiki/Message_Passing_Interface)
- [Rust basics and installation resources](https://www.rust-lang.org/learn/get-started)
- [Singularity resources](https://sylabs.io/docs/#singularity)
- [Rust mpi(rsmpi) github page](https://github.com/rsmpi/rsmpi)
- [rsmpi documentation](http://rsmpi.github.io/rsmpi/mpi/index.html)
- [more rsmpi examples](https://github.com/rsmpi/rsmpi/tree/master/examples)
- [one api installation link](https://software.intel.com/content/www/us/en/develop/tools/oneapi/base-toolkit/download.html)

## References and credits
- This repository is the work of student for a winter course of independent study under [Dr. Jaroslaw Zola](https://cse.buffalo.edu/~jzola/).
- The work was conducted in the college State University of New York at Buffalo. 
- The code was developed on a laptop and then later scaled and run using slurm scripts on the [UB CCR HPC cluster](http://www.buffalo.edu/ccr.html).
- The code from the examples of RSMPI github were referred for syntax and general idea of working with MPI using Rust. The github repository and link to examples are mentioned in resources above.

## Things remaining to do
- Figure out complete mpi crate support issues with default features in singularity container(weird errors which are still not fixed on container but works fine on VM)
- Add asserts in the code to ensure sanity of program. Currently manually tested and checked code is present(was done extensively for smaller sizes and readable message sizes before moving to larger message sizes)
- Check for malloc errors in C++ code(seems to occur when large number of ranks are involved and smaller message sizes, maybe something to do with the initialization of vectors)