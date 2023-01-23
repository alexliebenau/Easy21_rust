use std::thread;
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use ndarray::{ArcArray, Array2, Array3};

use num_cpus;

use crate::{framework, env};
use crate::framework::{Algorithm, D, P, A};

// impl Algorithm {
//     fn get_q_parallel(mut self, iterations: u64) -> Self {
//         const CPU_CORES: usize = num_cpus::get();
//
//         let iter_thread: i32 = iterations % CPU_CORES;
//         let remaining_iter: i32 = iterations.rem_euclid(CPU_CORES as i32);
//
//         let iter_array: [i32; CPU_CORES] = [iter_thread; CPU_CORES];
//         for i in 0..remaining_iter {
//             iter_array[i] += 1;
//         };
//     }
// }