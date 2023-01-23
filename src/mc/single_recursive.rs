use indicatif::ProgressBar;
use rand::seq::SliceRandom;
use crate::{framework, env};
use std::cmp::{max};
use ndarray::{Array2, Array3};
use crate::framework::{Algorithm, D, P, A};

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