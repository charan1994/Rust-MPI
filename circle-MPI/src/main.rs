extern crate mpi;

use mpi::point_to_point as p2p;
use mpi::traits::*;
use std::time::{Duration, Instant};

fn main() {
    let universe = mpi::initialize().unwrap(); //MPI COMM world setup equivalent 
    let world = universe.world();
    let size = world.size(); // number of processes which are running
    let rank = world.rank(); //rank of current process

    let next_rank = (rank + 1) % size; //next rank is either higher rank or 0 for the last rank
    let next_process = world.process_at_rank(next_rank);
    let previous_rank = (rank - 1) % size;
    let previous_process = world.process_at_rank(previous_rank);
    //previous rank is either lower rank or n-1 ie last rank for the first rank
    let message_size = 1024 * 1024; //this is the number of i32 ie 4 byte ints being sent as message

    let send_buffer = (1..).map(|x| rank * x + x).take(message_size).collect::<Vec<_>>();
    let mut receive_buffer = std::iter::repeat(-1).take(message_size).collect::<Vec<_>>();
    
    println!("Rank {} is sending the message {:?}",rank,send_buffer);
    world.barrier();
    let start = Instant::now();

    let status;
    {
        status = p2p::send_receive_into(&send_buffer[..], &next_process, &mut receive_buffer[..], &previous_process);
    }
    
    world.barrier();
    let duration = start.elapsed();
    println!("Rank {} received message: {:?}, status: {:?}",rank, receive_buffer, status);
    println!("Time spent in the code: {:?}",duration);

}
