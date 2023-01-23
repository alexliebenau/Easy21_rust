mod single_recursive;
mod parallel_recursive;


use crate::framework;


pub fn run(i: u64) -> framework::Algorithm {

   single_recursive::return_instance(i)
}

