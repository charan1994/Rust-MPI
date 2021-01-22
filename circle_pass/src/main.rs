extern crate mpi;

use mpi::point_to_point as p2p;
use mpi::traits::*;
use std::time::{Instant};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        println!("Invalid number of arguments. Need n")
        return
    }
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let size = world.size();
    let rank = world.rank();

    let next_rank = (rank + 1) % size;
    let next_process = world.process_at_rank(next_rank);
    let previous_rank = ((rank - 1)as i32).rem_euclid(size);
    let previous_process = world.process_at_rank(previous_rank);
    
    let message_size = &args[1].parse::<u64>().unwrap();

    let send_buffer = (1..).take(message_size).collect::<Vec<_>>();
    let mut receive_buffer = std::iter::repeat(-1).take(message_size).collect::<Vec<_>>();
    
    world.barrier();
    let start = Instant::now();
    
    p2p::send_receive_into(&send_buffer[..], &next_process, &mut receive_buffer[..], &previous_process);
    
    world.barrier();
    let duration = start.elapsed();

    if rank==0{
        println!("Size of the MPI_COMM_WORLD: {:?}",size);
        println!("Time spent in the code: {:?}",duration);
    }
}
