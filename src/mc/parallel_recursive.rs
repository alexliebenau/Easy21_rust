use std::borrow::BorrowMut;
use std::cmp::max;
use std::iter::Sum;
use std::thread;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use ndarray::{ArcArray, Array, array, Array2, Array3, ArrayBase, Ix3, OwnedRepr};
use rayon::prelude::*;

use num_cpus;
use num_traits::{FromPrimitive, Num};
use rand::seq::SliceRandom;
// use threadpool::ThreadPool;
// use rusty_pool:
// use rusty_pool::{JoinHandle, ThreadPool};

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
    get_q_parallel(iterations)
        .get_v()
}

struct ToWorker {
    q_hit: f32,
    q_stick: f32,
    n_hit: i32,
    n_stick: i32
}

// impl Sum for Vec<JoinHandle<Algorithm>> {
//     fn sum<I: Iterator>(iter: I) -> Self {
//         todo!()
//     }
// }


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

    // let mutarr_q = Arc::new(Mutex::new(Array3::<f32>::zeros((D, P, A))));
    // let mutarr_n = Arc::new(Mutex::new(Array3::<i32>::zeros((D, P, A))));

    println!("Detected {} CPU cores", cpu_cores);
    // let pool = ThreadPool::default();


    // let mut new_q = ArcArray::from(Array3::<Arc<Mutex<f32>>>::zeros((D, P, A)));
    // let mut new_n = ArcArray::from(Array3::<Arc<Mutex<i32>>>::zeros((D, P, A)));
    // let mut new_q: ArcArray<Arc<Mutex<f32>>, Ix3> = Default::default(); //[[[Arc::new(Mutex::new(0.0)); D];
    // let mut new_n: ArcArray<Arc<Mutex<i32>>, Ix3> = Default::default();
    // println!{"{:?}", new_q.shape()};
    // for _d in 0..D {
    //     for _p in 0..P {
    //         for _a in 0..A {
    //             new_q[[_d, _p, _a]] = Arc::new(Mutex::new(0.0));
    //             new_n[[_d,_p, _a]] = Arc::new(Mutex::new(0));
    //         }
    //     }
    // }
    let v_d: Vec<usize> =(0..D).collect();
    while iter_vec.len() > 0  {
        // let mut handle_vec = vec![];
        for _ in 0..cpu_cores {
            iter_vec.remove(0);
            // let mv_q =get_clone(&new_q);
            // let mv_n = get_clone(&new_n);
            // let handle = thread::spawn( move || worker(&mv_q, &mv_n));

            for _d in 0..D {
                thread::scope(|s| {
                    /*let handle = */s.spawn(|| {
                        // let _d: usize = *v_d.choose(&mut rand::thread_rng()).unwrap();
                        for _p in 0..P {
                            let hit = q_iter(&mutvec_n, &index_arr, 0, _d.clone(), _p); // add self. remove &self
                            let stick = q_iter(&mutvec_n, &index_arr, 1, _d.clone(), _p); // add self.

                            let action: usize = if hit > stick { 1 } else { 0 };
                            let q_i = *mutvec_q[index_arr[[_d.clone(), _p, action]]].lock().unwrap();

                            *mutvec_q[index_arr[[_d.clone(), _p, action]]].lock().unwrap() += (max(hit, stick) as f32 - q_i)
                                / *mutvec_n[index_arr[[_d.clone(), _p, action]]].lock().unwrap() as f32;
                        }
                    });
                    // handle_vec.push(handle);
                    // for handle in handle_vec{
                    //     handle.join().unwrap();
                    // }
                });
            }



            // for _d in 0..D {
            //     for _p in 0..P {
            //         let index_hit = index_arr[[_d, _p, 1]];
            //         let index_stick = index_arr[[_d, _p, 0]];
            //         let inp = ToWorker {
            //             q_hit: *Arc::clone(&mutvec_q[index_hit]).lock().unwrap(),
            //             n_hit: *Arc::clone(&mutvec_n[index_hit]).lock().unwrap(),
            //             q_stick: *Arc::clone(&mutvec_q[index_stick]).lock().unwrap(),
            //             n_stick: *Arc::clone(&mutvec_n[index_stick]).lock().unwrap(),
            //         };
            //
            //
            //         // (
            //         //     *Arc::clone(&mutvec_q[index_hit]).lock().unwrap(),
            //         //     *Arc::clone(&mutvec_n[index_hit]).lock().unwrap(),
            //         //     *Arc::clone(&mutvec_q[index_stick]).lock().unwrap(),
            //         //     *Arc::clone(&mutvec_n[index_stick]).lock().unwrap(),
            //         // )
            //         let handle = pool.evaluate(move || worker(inp, _d, _p));
            //         handle_vec.push(handle);
            //         // (new_q[[_d, _p, 0]], new_q[[_d, _p, 1]], new_n[[_d, _p, 0]], new_n[[_d, _p, 1]]) =
            //         //     handle.await_complete();
            //     }
            // }
            // let toW = Algorithm {
            //     q: Array3*Arc::clone(&mutarr_q).lock().unwrap(),
            //     v: Array2::<f32>::zeros((D, P, A)),
            //     n: *Arc::clone(&mutarr_n)
            // };
        }
        // for handle in handle_vec{
        //     handle.join().unwrap();
        // }
        // handle_vec.sum();
    }
    // for _d in 0..D {
    //     for _p in 0..P {
    //         // new_q[[_d, _p, 0]] = handle_vec[i].await_complete().q_stick;
    //         // new_q[[_d, _p, 1]] = handle_vec[i].await_complete().q_hit;
    //         // new_n[[_d, _p, 0]] = handle_vec[i].await_complete().n_stick;
    //         // new_n[[_d, _p, 1]] = handle_vec[i].await_complete().n_hit;
    //         (new_q[[_d, _p, 0]], new_q[[_d, _p, 1]], new_n[[_d, _p, 0]], new_n[[_d, _p, 1]]) =
    //             handle_vec.remove(i).await_complete();
    //         i += 1;
    //     }
    // }
    let mut q_out = Array3::<f32>::zeros((D, P, A));
    let mut n_out = Array3::<i32>::zeros((D, P, A));
    for _d in 0..D {
        for _p in 0..P {
            for _a in 0..A {
                // q_out[[_d, _p, _a]] = *new_q[[_d, _p, _a]].lock().unwrap();
                // n_out[[_d, _p, _a]] = *new_n[[_d, _p, _a]].lock().unwrap();
                q_out[[_d, _p, _a]] = *mutvec_q[index_arr[[_d, _p, _a]]].lock().unwrap();
                n_out[[_d, _p, _a]] = *mutvec_n[index_arr[[_d, _p, _a]]].lock().unwrap();
            }
        }
    }

    Algorithm {
        q: q_out, //*mutex_q.lock().unwrap(),
        v: Array2::<f32>::zeros((D, P)),
        n: n_out //mutex_n.into_inner().unwrap()
    }
}
/* gonna cmt this out for a while
fn get_clone<T: Default>(i: &ArcArray<Arc<Mutex<T>>, Ix3>) -> ArcArray<Arc<Mutex<T>>, Ix3> {
    let mut out: ArcArray<Arc<Mutex<T>>, Ix3> = Default::default();
    for _d in 0..D {
        for _p in 0..P {
            for _a in 0..A {
                out[[_d, _p, _a]] = Arc::clone(&i[[_d, _p, _a]]);
            }
        }
    }
    out
}

fn init_zero<T: Num>(/* d: usize, p: usize, a: usize*/)  -> ArcArray<Arc<Mutex<T>>, Ix3> {
    let mut v1 = Vec::with_capacity(A);
    for _ in 0..A {
        v1.push(Arc::new(Mutex::new(T::zero())));
    }
    let a1 = Array::from_vec(v1);

   let mut v2 = Vec::with_capacity(P);
    for _ in 0..P {
        v2.push(a1.clone());
    }
    let a2 = Array::from_vec(v2);

    let mut v3 =Vec::with_capacity(D);
    for _ in 0..D {
        v3.push(v2.clone())
    }

   Array3::from_shape_vec((D, P, A), v3).unwrap().into_shared()
}


fn worker(q: &ArcArray<Arc<Mutex<f32>>, Ix3>,
            n: &ArcArray<Arc<Mutex<i32>>, Ix3>) { //-> ((f32, f32, i32, i32) {
    for _d in 0..D {
        for _p in 0..P {
            let hit = q_iter(n, 0, _d, _p); // add self. remove &self
            let stick = q_iter(n, 1, _d, _p); // add self.

            let action: usize = if hit > stick { 1 } else { 0 };
            let q_i = *q[[_d, _p, action]].lock().unwrap();

            *q[[_d, _p, action]].lock().unwrap() += (max(hit, stick) as f32 - q_i )
                / *n[[_d, _p, action]].lock().unwrap() as f32;
            }
        }
    }
    //
    // // inp
    // (inp.q_hit, inp.q_stick, inp.n_hit, inp.n_stick)
// }

// fn q_iter_arc(n: ArcArray<Arc<Mutex<i32>>, Ix3>, action: usize, _d: usize, _p: usize) -> i16
// {
//     *n[[_d, _p, action]].lock().unwrap() += 1;
//     let mut game: env::Game = env::Game {
//         dealer_sum: _d as i16,
//         player_sum: _p as i16,
//     };
//     let ret: env::Gamestate = game.step(action != 0);
//     return if ret.is_terminal {
//         ret.reward as i16
//     } else {
//         let n_tot= *n[[_d, _p, 0]].lock().unwrap() + *n[[_d, _p, 1]].lock().unwrap();
//         return if framework::greedy(n_tot) {
//             let hit = q_iter_arc(n, 0, ret.dealer as usize, ret.player as usize);
//             let stick = q_iter_arc(n, 1, ret.dealer as usize, ret.player as usize);
//             max(hit, stick)
//         } else {
//             let choice = *vec![0, 1].choose(&mut rand::thread_rng()).unwrap();
//             ret.reward as i16 + q_iter(n, choice, ret.dealer as usize, ret.player as usize) as i16
//         }
//     }
// }
*/

fn q_iter(
    /* n: &ArcArray<Arc<Mutex<i32>>, Ix3>,*/ n: &Vec<Arc<Mutex<i32>>>, ind: &ArrayBase<OwnedRepr<usize>, Ix3>,
    action: usize, _d: usize, _p: usize) -> i16
{
    // if action != 0 {
    //     n_hit += 1;
    // } else{
    //     n_stick += 1;
    // }
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