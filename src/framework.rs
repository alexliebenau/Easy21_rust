use ndarray::{Array2, Array3};
use rand::prelude::*;

pub static D: usize = 10;
pub static P: usize = 22;
pub static A: usize = 2;

#[derive(Clone, Default)]
pub struct Algorithm {
    pub q: Array3<f32>,
    pub v: Array2<f32>,
    pub n: Array3<i32>,
}

pub trait AlgMethods {
    fn get_q(self, iterations: i32) -> Self;

    // fn q_iter(self, a: usize, d: usize, p: usize) -> (Algorithm, i16, bool);

    fn new() -> Algorithm {
        Algorithm {
            q: Array3::<f32>::zeros((D, P, A)),
            v: Array2::<f32>::zeros((D, P)),
            n: Array3::<i32>::zeros((D, P, A))
        }
    }

    fn return_instance (i: i32) -> Algorithm {
        Algorithm::new().get_q(i).get_v()
    }
}

impl Algorithm {
    pub fn get_v(mut self) -> Self {
        for _d in 0..D-1 {
            for _p in 0..P - 1 { // get V* = max Q*(s, a)
                self.v[[_d, _p]] = self.q[[_d, _p, 0]].max(self.q[[_d, _p, 1]]);
            }
        }
        return self
    }

    pub fn incr_n(mut self, d: usize, p: usize, a: usize)  -> Self {
        self.n[[d, p, a]] += 1;
        return self
    }
}

pub fn greedy(n: i32) -> bool {
    // Define eps-greedy behaviour (see slides)
    // choose with a varying probability between taking the best or a random action
    // Takes number of times current state has been visited as input (=self.N[[d,p0]] + self.N[[d,p,1]])
    // return true if going greedy, return false if going random
    let n_0: f32 = 100.0;
    let eps: f32 = n_0 / (n as f32 + n_0); // epsilon for greedy-ness
    let choice = [(true, 1.0 - eps / 2.0), (false, eps / 2.0)];
    let mut rng = thread_rng();
    return choice.choose_weighted(&mut rng, |item| item.1).unwrap().0;
}
