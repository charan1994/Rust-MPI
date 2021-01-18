extern crate mpi;

use mpi::traits::*;
use std::time::{Instant};

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let size = world.size();
    let rank = world.rank();
    let message_size: u32 = 1024 * 1024;

    let send_buffer = (1..).map(|x| rank * x + x).take(message_size as usize).collect::<Vec<_>>();
    let mut receive_buffer = std::iter::repeat(-1).take(message_size as usize).collect::<Vec<_>>();

    world.barrier();
    let start = Instant::now();

    world.all_to_all_into(&send_buffer[..],&mut receive_buffer[..]);

    world.barrier();
    let duration = start.elapsed();

    if rank==0{
        println!("Size of the MPI_COMM_WORLD: {:?}",size);
        println!("Time spent in the code: {:?}",duration);
    }

}
