use ndarray::{Array, Ix3, Ix2};
use rand::prelude::*;

// for binary array write
use ndarray_npy::{WriteNpyExt, WriteNpyError};
use std::fs::File;
use std::io::BufWriter;
use std::ops::{Add, Div};
use num_traits::AsPrimitive;
use num_traits::real::Real;

pub static D: usize = 10;
pub static P: usize = 22;
pub static A: usize = 2;

#[derive(Clone)]
pub struct Algorithm {
    pub q: Array<f32, Ix3>,
    pub v: Array<f32, Ix2>,
    pub n: Array<i32, Ix3>,
}

// pub trait AlgMethods<T, U: PartialOrd>
//     where
//         T: Copy,
//
// {
//     // fn get_q<I: PartialOrd>(self, iterations: I) -> Self;
//
//     fn new() -> Algorithm<T>;
// }

impl Algorithm {
    // fn return_instance<I: PartialOrd> (self, iterations: I) -> Box<Self> {
    //     self.new()
    //         .get_q(iterations)
    //         .get_v()
    // }

    pub fn write_values(&self, path_to: &str) -> Result<(), WriteNpyError> {
        self.q.write_npy(set_buf(&path_to, "q.npy"))?;
        self.v.write_npy(set_buf(&path_to, "v.npy"))?;
        self.n.write_npy(set_buf(&path_to, "n.npy"))?;

        Ok(())
    }

    pub(crate) fn get_v(mut self) -> Algorithm {
        for _d in 0..D {
            for _p in 0..P { // get V* = max Q*(s, a)
                self.v[[_d, _p]] = self.q[[_d, _p, 0]].max(self.q[[_d, _p, 1]]);
            }
        }
        return self
    }
}

pub fn greedy(n: i32) -> bool {
    // Define eps-greedy behaviour (see slides)
    // choose with a varying probability between taking the best or a random action
    // Takes number of times current state has been visited as input (=self.N[[d,p0]] + self.N[[d,p,1]])
    // return true if going greedy, return false if going random
    let n_0 = 100.0;
    let eps = n_0 / (n as f64 + n_0); // epsilon for greedy-ness
    let choice = [(true, 1.0 - eps / 2.0), (false, eps / 2.0)];
    let mut rng = thread_rng();
    return choice.choose_weighted(&mut rng, |item| item.1).unwrap().0;
}

fn set_buf(path_prefix: &str, path: &str) -> BufWriter<File> {
    let p = format!("{}{}", path_prefix, path);
    BufWriter::new(File::create(p).unwrap())
}

// impl<T, U: PartialOrd + AlgMethods> Algorithm<T, U> {
//     pub fn get_v(mut self) -> Self {
//         for _d in 0..D {
//             for _p in 0..P { // get V* = max Q*(s, a)
//                 self.v[[_d, _p]] = &self.q[[_d, _p, 0]].max(&self.q[[_d, _p, 1]]);
//             }
//         }
//         return self
//     }
//
//     pub fn write_values(self, path_to: &str) -> Result<(), WriteNpyError> {
//         self.q.write_npy(set_buf(&path_to, "q.npy"))?;
//         self.v.write_npy(set_buf(&path_to, "v.npy"))?;
//         self.n.write_npy(set_buf(&path_to, "n.npy"))?;
//
//         Ok(())
//     }
// }