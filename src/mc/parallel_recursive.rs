use std::borrow::BorrowMut;
use std::cmp::max;
use std::thread;
use std::sync::{Arc, Mutex};
use ndarray::{ArcArray, Array, array, Array2, Array3, ArrayBase, Ix3, OwnedRepr};
use rayon::prelude::*;

use num_cpus;
use rand::seq::SliceRandom;
use threadpool::ThreadPool;

use crate::{framework, env};
use crate::framework::{Algorithm, D, P, A};

// pub fn return_instance(iterations: u64) -> Algorithm {
//     Algorithm {
//         q: ArcArray::from(Array3::<f32>::zeros((D, P, A))).to_owned(),
//         v: ArcArray::from(Array2::<f32>::zeros((D, P))).to_owned(),
//         n: ArcArray::from(Array3::<i32>::zeros((D, P, A))).to_owned()
//     }
//         .get_q_parallel(iterations)
//         .get_v()
// }
pub fn return_instance(iterations: u64) -> Algorithm {
    framework::get_default()
        .get_q_parallel(iterations)
        .get_v()
}

// struct Parstruct<R, S, T>  {
//     q: R,
//     v: S,
//     n: T
// }

impl Algorithm {
    fn get_q_parallel(mut self, iterations: u64) -> Self {
        let CPU_CORES: u64 = num_cpus::get() as u64;
        //
        // let iter_thread = iterations % CPU_CORES as u64;
        // let remaining_iter= iterations.rem_euclid(CPU_CORES);
        //
        // let iter_array: [u64; CPU_CORES] = [iter_thread; CPU_CORES];
        // for i in 0..remaining_iter {
        //     iter_array[i] += 1;
        // };
        let mut mutvec_q: Vec<Arc<Mutex<f32>>> = vec![];
        let mut mutvec_n: Vec<Arc<Mutex<i32>>> = vec![];
        let mut index_arr = Array3::<usize>::zeros((D, P, A));
        let mut i = 0;

        for _d in 0..D {
            for _p in 0..P {
                for _a in 0..A {
                    index_arr[[_d, _p, _a]] = i;
                    mutvec_q.push(Arc::new(Mutex::new(self.q[[_d, _p, _a]])));
                    mutvec_n.push(Arc::new(Mutex::new(self.n[[_d, _p, _a]])));
                    i += 1;
                }
            }
        }

        // let mutex_q = Arc::new(Mutex::new(self.q));
        // // let mutex_v = Arc::new(Mutex::new(self.v));
        // let mutex_n = Arc::new(Mutex::new(self.n));

        // let mut handle_vec = vec![];

        // let teststruct = Algorithm{
        //     q: mutex_q,
        //     v: mutex_v,
        //     n: mutex_n
        // };
        println!("Detected {} CPU cores", CPU_CORES);
        let pool = ThreadPool::new(CPU_CORES as usize);

        for _ in 0..iterations {
            for _d in 0..D {
                for _p in 0..P {
                    let index_hit = index_arr[[_d, _p, 1]];
                    let index_stick = index_arr[[_d, _p, 0]];
                    let q_hit = Arc::clone(&mutvec_q[index_hit]);
                    let n_hit = Arc::clone(&mutvec_n[index_hit]);
                    let q_stick = Arc::clone(&mutvec_q[index_stick]);
                    let n_stick = Arc::clone(&mutvec_n[index_stick]);

                    // let handle = thread::spawn(move ||
                    //     worker((q_i.lock().unwrap()).borrow_mut(), n_i.lock().unwrap().borrow_mut())
                    // );
                    // handle_vec.push(handle);

                    pool.execute(move ||
                        worker(
                            *q_hit.lock().unwrap(),
                            *q_stick.lock().unwrap(),
                            *n_hit.lock().unwrap(),
                            *n_stick.lock().unwrap(),
                            _d, _p
                        )
                    );
                }
            }
        }
        println!("Active threads: {}", pool.active_count());
        pool.join();

        Algorithm {
            q: Array3::zeros((D, P, A)), //*mutex_q.lock().unwrap(),
            v: self.v,
            n: Array3::zeros((D, P, A)) //mutex_n.into_inner().unwrap()
        }
    }
    // 
    // fn get_q_rayon(mut self, iterations: i32) -> Self {
    // 
    //     (0..iterations).into_par_iter_mut().map(
    //         |x| get_q_single(x)
    //     )
    // }
}


fn worker(mut q_hit: f32,
          mut q_stick: f32,
          mut n_hit: i32,
          mut n_stick: i32,
            _d: usize,
            _p: usize)  {

    // let (mut hit, mut stick): (i16, i16) = (0, 0);
    // n += 1;
    let hit = q_iter(n_hit, n_stick, 0, _d, _p); // add self. remove &self
    let stick = q_iter(n_hit, n_stick, 1, _d, _p); // add self.

    let n_res = if hit > stick { n_hit } else { n_stick };
    let q_res = if hit > stick { q_hit } else { q_stick };

    if hit > stick {
        q_hit += (hit as f32 - q_hit)
            / n_hit as f32;
        // n_hit += 1;
    } else {
        q_stick += (stick as f32 - q_stick)
            / n_stick as f32;
        // n_stick += 1;
}

}

fn q_iter(mut n_hit: i32, mut n_stick: i32, action: usize, _d: usize, _p: usize) -> i16
{
    if action != 0 {
        n_hit += 1;
    } else{
        n_stick += 1;
    }

    let mut game: env::Game = env::Game {
        dealer_sum: _d as i16,
        player_sum: _p as i16,
    };
    let ret: env::Gamestate = game.step(action != 0);
    return if ret.is_terminal {
        ret.reward as i16
    } else {
        let n_tot= n_hit + n_stick;
        return if framework::greedy(n_tot) {
            let hit = q_iter(n_hit, n_stick, 0, ret.dealer as usize, ret.player as usize);
            let stick = q_iter(n_hit, n_stick, 1, ret.dealer as usize, ret.player as usize);
            max(hit, stick)
        } else {
            let choice = *vec![0, 1].choose(&mut rand::thread_rng()).unwrap();
            ret.reward as i16 + q_iter(n_hit, n_stick, choice, ret.dealer as usize, ret.player as usize) as i16
        }
    }
}