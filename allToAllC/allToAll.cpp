#include <vector>
#include <mpi.h>
#include <algorithm>
#include <iostream>
#include <random>

int main(int argc, char *argv[])
{
    int size, rank;

    MPI_Init(&argc, &argv);
    MPI_Comm_size(MPI_COMM_WORLD, &size);
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);

    unsigned int messageSize = 1032 * 1032;

    std::vector<int> send_buffer(messageSize);
    std::vector<int> receive_buffer(messageSize);

    std::iota(send_buffer.begin(), send_buffer.end(), 1);

    MPI_Barrier(MPI_COMM_WORLD);
    auto t0 = MPI_Wtime();

    MPI_Alltoall(send_buffer.data(), 1, MPI_INT, receive_buffer.data(), 1, MPI_INT, MPI_COMM_WORLD);

    MPI_Barrier(MPI_COMM_WORLD);
    auto t1 = MPI_Wtime();

    for (std::vector<int>::const_iterator i = receive_buffer.begin(); i != receive_buffer.end(); ++i)
        std::cout << *i << ", ";

    if (rank == 0)
    {        
        std::cout << std::endl;
        std::cout << "Size of the MPI_COMM_WORLD: " + std::to_string(size) << std::endl;
        std::cout << "Time spent in code: " + std::to_string(t1 - t0) << std::endl;
    }

    return MPI_Finalize();
}