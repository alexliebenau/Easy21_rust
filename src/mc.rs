mod single_recursive;
mod parallel_recursive;
// mod rayon_recursive;


use crate::framework::{Algorithm, yeet};


pub fn run(i: u64, version: &str) -> Algorithm {

   match version {
      "s" => single_recursive::return_instance(i),
      "p" => parallel_recursive::return_instance(i),
      // "r" => rayon_recursive::return_instance(i),
      _ => yeet()
   }

}

