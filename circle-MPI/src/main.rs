extern crate mpi;

use mpi::point_to_point as p2p;
use mpi::traits::*;

fn main() {
    let universe = mpi::initialize().unwrap(); //MPI COMM world setup equivalent 
    let world = universe.world();
    let size = world.size(); // number of processes which are running
    let rank = world.rank(); //rank of current process

    let next_rank = if rank + 1 < size {rank + 1} else {0}; //next rank is either higher rank or 0 for the last rank
    let next_process = world.process_at_rank(next_rank);
    let previous_rank = if rank > 0 {rank - 1} else {size -1}; 
    let previous_process = world.process_at_rank(previous_rank);
    //previous rank is either lower rank or n-1 ie last rank for the first rank

    let send_buffer = 10;
    let mut receive_buffer = -1;
    
    println!("Rank {} is sending the message {:?}",rank,send_buffer);
    world.barrier();

    let status;
    {
        status = p2p::send_receive_into(&send_buffer, &next_process, &mut receive_buffer, &previous_process);
    }

    println!("Rank {} received message: {:?}, status: {:?}",rank, receive_buffer, status);
    world.barrier();

}
