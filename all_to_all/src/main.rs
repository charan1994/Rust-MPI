extern crate mpi;

use mpi::traits::*;
use std::time::{Instant};

fn main() {
    let universe = mpi::initialize().unwrap(); //MPI COMM world setup equivalent 
    let world = universe.world();
    let size = world.size(); // number of processes which are running
    let rank = world.rank(); //rank of current process
    let message_size: u32 = 1024 * 1024; //this is the number of i32 ie 4 byte ints being sent as message

    let send_buffer = (1..).map(|x| rank * x + x).take(message_size as usize).collect::<Vec<_>>();
    let mut receive_buffer = std::iter::repeat(-1).take(message_size as usize).collect::<Vec<_>>();

    // println!("Rank {} is sending the message {:?}",rank,send_buffer); you can use this just while debugging for smaller size message sizes

    world.barrier();
    let start = Instant::now();

    world.all_to_all_into(&send_buffer[..],&mut receive_buffer[..]);

    world.barrier();
    let duration = start.elapsed();

    // println!("Rank {} received message: {:?}",rank, receive_buffer); you can use this just while debugging for smaller size message sizes
    if (rank==0){
        println!("Time spent in the code: {:?}",duration);
    }

}
