use std::cmp::max;
use std::thread;
use std::sync::{Arc, Mutex};
use ndarray::{Array2, Array3, ArrayBase, Ix3, OwnedRepr};

use num_cpus;
use rand::seq::SliceRandom;

use crate::{framework, env};
use crate::framework::{Algorithm, D, P, A};

pub fn return_instance(iterations: u64) -> Algorithm {
    get_q_parallel(iterations)
        .get_v()
}


fn get_q_parallel(iterations: u64) -> Algorithm {

    let mut iter_vec: Vec<u64> = (0..iterations).collect();

    let cpu_cores: u64 = num_cpus::get() as u64;

    let mut mutvec_q: Vec<Arc<Mutex<f32>>> = vec![];
    let mut mutvec_n: Vec<Arc<Mutex<i32>>> = vec![];
    let mut index_arr = Array3::<usize>::zeros((D, P, A));
    let mut i = 0;

    for _d in 0..D {
        for _p in 0..P {
            for _a in 0..A {
                index_arr[[_d, _p, _a]] = i;
                mutvec_q.push(Arc::new(Mutex::new(0.0)));
                mutvec_n.push(Arc::new(Mutex::new(0)));
                i += 1;
            }
        }
    }

    println!("Detected {} CPU cores", cpu_cores);
    while iter_vec.len() > 0  {
        for _ in 0..cpu_cores {
            iter_vec.remove(0);
            for _d in 0..D {
                thread::scope(|s| {
                    s.spawn(|| {
                        for _p in 0..P {
                            let hit = q_iter(&mutvec_n, &index_arr, 0, _d.clone(), _p); // add self. remove &self
                            let stick = q_iter(&mutvec_n, &index_arr, 1, _d.clone(), _p); // add self.

                            let action: usize = if hit > stick { 1 } else { 0 };
                            let q_i = *mutvec_q[index_arr[[_d.clone(), _p, action]]].lock().unwrap();

                            *mutvec_q[index_arr[[_d.clone(), _p, action]]].lock().unwrap() += (max(hit, stick) as f32 - q_i)
                                / *mutvec_n[index_arr[[_d.clone(), _p, action]]].lock().unwrap() as f32;
                        }
                    });
                });
            }
        }
    }
    let mut q_out = Array3::<f32>::zeros((D, P, A));
    let mut n_out = Array3::<i32>::zeros((D, P, A));
    for _d in 0..D {
        for _p in 0..P {
            for _a in 0..A {
                q_out[[_d, _p, _a]] = *mutvec_q[index_arr[[_d, _p, _a]]].lock().unwrap();
                n_out[[_d, _p, _a]] = *mutvec_n[index_arr[[_d, _p, _a]]].lock().unwrap();
            }
        }
    }

    Algorithm {
        q: q_out,
        v: Array2::<f32>::zeros((D, P)),
        n: n_out
    }
}

fn q_iter(
    n: &Vec<Arc<Mutex<i32>>>, ind: &ArrayBase<OwnedRepr<usize>, Ix3>,
    action: usize, _d: usize, _p: usize) -> i16
{
    *n[ind[[_d, _p, action]]].lock().unwrap() += 1;

    let mut game: env::Game = env::Game {
        dealer_sum: _d as i16,
        player_sum: _p as i16,
    };
    let ret: env::Gamestate = game.step(action != 0);
    return if ret.is_terminal {
        ret.reward as i16
    } else {
        let n_tot= *n[ind[[_d, _p, 0]]].lock().unwrap() + *n[ind[[_d, _p, 1]]].lock().unwrap();
        return if framework::greedy(n_tot) {
            let hit = q_iter(n, ind, 0, ret.dealer as usize, ret.player as usize);
            let stick = q_iter(n, ind, 1, ret.dealer as usize, ret.player as usize);
            max(hit, stick)
        } else {
            let choice = *vec![0, 1].choose(&mut rand::thread_rng()).unwrap();
            ret.reward as i16 + q_iter(n, ind, choice, ret.dealer as usize, ret.player as usize) as i16
        }
    }
}