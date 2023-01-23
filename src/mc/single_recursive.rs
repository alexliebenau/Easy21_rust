use std::borrow::Borrow;
use indicatif::ProgressBar;
use rand::seq::SliceRandom;
use crate::{framework, env};
use std::cmp::{max};
use ndarray::{Array, Array2, Array3, Dim, Dimension, Ix2, Ix3, NdIndex};
use num_traits::{FromPrimitive, Zero};
use num::cast::{ToPrimitive, AsPrimitive};
// use num::traits::{NumAssignOps, NumOps, NumAssign};
use num::{Integer, Float, Num};
use std::ops::{Add, AddAssign, Sub, Div, SubAssign};
use crate::framework::{Algorithm, D, P, A};


// struct single as Algorithm::<f32, i32>;


// pub fn single_recursive() -> impl AlgMethods<T> {
//     Algorithm {
//         q: Array3::<f32>::zeros((D, P, A)),
//         v: Array2::<f32>::zeros((D, P)),
//         n: Array3::<i32>::zeros((D, P, A))
//     }
// }

// pub struct single_recursive<T, U> {
//     pub q: Array<T, Ix3>,
//     pub v: Array<T, Ix2>,
//     pub n: Array<U, Ix3>,
// }

// type single_recursive<T, U> = Algorithm<T, U>;

// impl<T, U> AlgMethods<T, U> for single_recursive<T, U>
//     where
//         T: Copy,
//         U: Copy + PartialOrd
//
// {
pub fn return_instance(iterations: u64) -> Algorithm {
    Algorithm {
                    q: Array3::<f32>::zeros((D, P, A)),
                    v: Array2::<f32>::zeros((D, P)),
                    n: Array3::<i32>::zeros((D, P, A))
                }
        .get_q_single(iterations)
        .get_v()
}

impl Algorithm {
    fn get_q_single(mut self, iterations: u64) -> Self {
        let bar: ProgressBar = ProgressBar::new(iterations);
        for _i in 0..iterations {
            for _d in 0..D {
                for _p in 0..P {
                    // let (mut hit, mut stick): (i16, i16) = (0, 0);
                    let hit = q_iter(&mut self, 0, _d, _p); // add self. remove &self
                    let stick = q_iter(&mut self, 1, _d, _p); // add self.

                    let action: usize = if hit > stick { 1 } else { 0 };

                    let n = self.n[[_d, _p, action]];
                    let q = self.q[[_d, _p, action]];

                    self.n[[_d, _p, action]] += 1;
                    self.q[[_d, _p, action]] += (max(hit, stick) as f32 - q)
                        / n as f32;
                }
            }
            bar.inc(1);
        }
        bar.finish();
        return self
    }
}

    // fn new() -> Self {
    //     {
    //         Algorithm {
    //             // q: ArcArray::from(Array3::<f32>::zeros((D, P, A))).to_owned(),
    //             q: Array3::<T>::zeros((D, P, A)),
    //             v: Array2::<T>::zeros((D, P)),
    //             n: Array3::<U>::zeros((D, P, A))
    //         }
    //     }
    // }
// }

// fn get_q(mut inst: impl AlgMethods, iterations: i32) -> i16 {//Algorithm<f32, i32> {
//     let bar: ProgressBar = ProgressBar::new(iterations as u64);
//     for _i in 0..iterations {
//         for _d in 0..D {
//             for _p in 0..P {
//                 // let (mut hit, mut stick): (i16, i16) = (0, 0);
//                 let hit = q_iter(&mut inst, 0, _d, _p); // add self. remove &self
//                 let stick = q_iter(&mut inst, 1, _d, _p); // add self.
//
//                 let action: usize = if hit > stick{1} else{0};
//                 inst.n[[_d, _p, action]] += 1;
//                 inst.q[[_d, _p, action]] += (max(hit, stick) as f32 - inst.q[[_d, _p, action]])
//                     / inst.n[[_d, _p, action]] as f32;
//             }
//         }
//         bar.inc(1);
//     }
//     bar.finish();
//     return inst
// }
//
// fn new(inst: impl AlgMethods) -> Algorithm<f32, i32> {
//     return inst
// }
//
// fn return_instance(i: i32) -> Algorithm<f32, i32> {
//     get_q(new(single_recursive()), i).get_v()
// }

fn q_iter(inst: &mut Algorithm, action: usize, _d: usize, _p: usize) -> i16
{
    inst.n[[_d, _p, action]] += 1;
    let mut game: env::Game = env::Game {
        dealer_sum: _d as i16,
        player_sum: _p as i16,
    };
    let ret: env::Gamestate = game.step(action != 0);
    return if ret.is_terminal {
        ret.reward as i16
    } else {
        let n_tot= &inst.n[[_d, _p, 0]] + &inst.n[[_d, _p, 0]];
        return if framework::greedy(n_tot) {
            let hit = q_iter(inst, 0, ret.dealer as usize, ret.player as usize);
            let stick= q_iter(inst, 1, ret.dealer as usize, ret.player as usize);
            max(hit, stick)
        } else {
            let choice = *vec![0, 1].choose(&mut rand::thread_rng()).unwrap();
            ret.reward as i16 + q_iter(inst, choice, ret.dealer as usize, ret.player as usize) as i16
        }
    }
}

// pub fn get_inst<T: Copy, U: Copy, I>(iterations: I) -> Algorithm<T, U> {
//     Algorithm::return_instance(iterations)
// }