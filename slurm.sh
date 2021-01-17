#!/bin/bash

#SBATCH --mem=32000 --constraint=CPU-E5-2660
#SBATCH --account=jzola
#SBATCH --partition=general-compute
#SBATCH --qos=general-compute
#SBATCH --exclusive
#SBATCH --job-name="Rust"
#SBATCH --output=%j.stdout
#SBATCH --error=%j.stderr
#SBATCH --nodes=1
#SBATCH --ntasks-per-node=8
#SBATCH --time=01:00:00

date
module load intel-mpi/2020.1
# singularity is already part of runtime so need to import new modules
echo "Running with 1 node"
srun --mpi=pmi2 singularity run --bind $I_MPI_ROOT/intel64:/opt/intel/oneapi/mpi/2021.1.1 run-test.simg