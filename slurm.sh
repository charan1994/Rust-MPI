#!/bin/bash

#SBATCH --mem=64000
#SBATCH --account=jzola
#SBATCH --partition=planex
#SBATCH --qos=planex
#SBATCH --exclusive
#SBATCH --clusters=faculty
#SBATCH --job-name="Rust-MPI"
#SBATCH --output=%j.stdout
#SBATCH --error=%j.stderr
#SBATCH --nodes=1
#SBATCH --ntasks-per-node=8
#SBATCH --time=01:00:00

date
module load intel-mpi/2020.1
# singularity is already part of runtime so no need to import new modules
echo "Running with 1 node and 8 ranks"
# size of message is message_size * 4 
# 4b 32b 64b 128b 256b 512b 1kb 2kb 4kb 8kb 16kb 32kb 64kb 128kb 256kb 512kb 1mb 2mb 4mb 8mb 16mb 32mb 64mb 128mb
message_sizes=(1 8 16 32 64 128 256 512 1024 2048 4096 8192 16384 32768 65536 131072 262144 524288 1048576 2097152 4194304 8388608 16777216 33554432)

for message_size in ${message_sizes[@]}; do
    for turn in {1..10}; do
        echo "running all_to_all rust code for turn: $turn and message size: $message_size"
        srun --mpi=pmi2 singularity run --bind $I_MPI_ROOT/intel64:/opt/intel/oneapi/mpi/2021.1.1 all_to_all.simg $message_size
    done
done

for message_size in ${message_sizes[@]}; do
    for turn in {1..10}; do
        echo "running all_to_all c++ code for turn: $turn and message size: $message_size"
        srun --mpi=pmi2 singularity run --bind $I_MPI_ROOT/intel64:/opt/intel/oneapi/mpi/2021.1.1 all_to_all_cxx.simg $message_size
    done
done

for message_size in ${message_sizes[@]}; do
    for turn in {1..10}; do
        echo "running circle pass rust code for turn: $turn and message size: $message_size"
        srun --mpi=pmi2 singularity run --bind $I_MPI_ROOT/intel64:/opt/intel/oneapi/mpi/2021.1.1 circle_pass.simg $message_size
    done
done

for message_size in ${message_sizes[@]}; do
    for turn in {1..10}; do
        echo "running circle pass c++ code for turn: $turn and message size: $message_size"
        srun --mpi=pmi2 singularity run --bind $I_MPI_ROOT/intel64:/opt/intel/oneapi/mpi/2021.1.1 circle_pass_cxx.simg $message_size
    done
done
