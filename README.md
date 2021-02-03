# MPI support in Rust
This repository contains basic code and some runtimes to test how well MPI is currently supported in Rust.
The project uses [rsmpi crate](https://github.com/rsmpi/rsmpi) paired with the Intel oneAPI MPI.

The repository provides basic examples testing send-receive and all-to-all communication patterns for fixed-size messages. It also uses barriers to synchronize timing of the programs.

The development of the code was done using the Intel oneAPI HPC kit (version 2021.1.0). We provide definition to build a [Singularity container](https://sylabs.io/docs/#singularity), which can be deployed in an HPC center. Please refer to the comments in the definition file for more information.

The given examples do not use rsmpi, features such as `user-operations` and `derive`.

## Content
The repository provides the following components:

- Code for all-to-all performance test in Rust and C++ (`all_to_all` and `all_to_all_cxx`).
- Code for point-to-point over the ring performance test in Rust and C++ (`circle_pass` and `circle_pass_cxx`).
- Logs from SLURM jobs run at different configurations (`logs`).
- Documents and jupyter notebook to compare the Rust and C++ runtimes, storage and other stats (`reports`).

## Requirements
1. Intel MPI compiler and related tools (either oneAPI or Intel-MPI based installs).
2. Libfabric.
3. Rust.
4. Clang compiler for some Rust crates that are built from source.

## Resources
- [MPI standard introduction](https://en.wikipedia.org/wiki/Message_Passing_Interface)
- [Rust basics and installation resources](https://www.rust-lang.org/learn/get-started)
- [Singularity resources](https://sylabs.io/docs/#singularity)
- [Rust mpi (rsmpi) GitHub page](https://github.com/rsmpi/rsmpi)
- [rsmpi documentation](http://rsmpi.github.io/rsmpi/mpi/index.html)
- [more rsmpi examples](https://github.com/rsmpi/rsmpi/tree/master/examples)
- [Intel oneAPI installation link](https://software.intel.com/content/www/us/en/develop/tools/oneapi/base-toolkit/download.html)

## References and credits
- This repository is the work of student for a winter course of independent study under [Dr. Jaroslaw Zola](https://cse.buffalo.edu/~jzola/).
- The work was conducted at the University at Buffalo. 
- The code was developed on a laptop, and the resulting containers were deployed and ran at scale under control of SLURM on the [UB CCR HPC cluster](http://www.buffalo.edu/ccr.html).
- The examples from the rsmpi GitHub repository were used for reference for syntax and general idea of working with MPI using Rust. The GitHub repository and links to the examples are listed in the resources above.

## Things to do
- Resolve the problems with using complete rsmpi crate when building Singularity image (currently, we get unexpected errors when building container, which do not manifest themselves when compiling the project outside of the container).
