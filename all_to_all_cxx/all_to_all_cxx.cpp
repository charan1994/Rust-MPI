#include <mpi.h>
#include <iostream>
#include <numeric>
#include <vector>


int main(int argc, char *argv[])
{
    int size, rank;

    MPI_Init(&argc, &argv);
    MPI_Comm_size(MPI_COMM_WORLD, &size);
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);

    if (argc < 2) {
        if (rank == 0) std::cout << "usage: " << argv[0] << " n" << std::endl;
        return MPI_Finalize();
    }

    unsigned long messageSize = std::stoul(argv[1]);

    std::vector<int> send_buffer(size * messageSize);
    std::vector<int> receive_buffer(size * messageSize);

    std::iota(send_buffer.begin(), send_buffer.end(), 1);

    MPI_Barrier(MPI_COMM_WORLD);
    auto t0 = MPI_Wtime();

    MPI_Alltoall(send_buffer.data(), messageSize, MPI_INT, receive_buffer.data(), messageSize, MPI_INT, MPI_COMM_WORLD);

    MPI_Barrier(MPI_COMM_WORLD);
    auto t1 = MPI_Wtime();

    if (rank == 0)
    {
        std::cout << "Size of the MPI_COMM_WORLD: " + std::to_string(size) << std::endl;
        std::cout << "Time spent in code: " + std::to_string((t1 - t0) * 1000) << "ms" << std::endl;
    }

    return MPI_Finalize();
}
