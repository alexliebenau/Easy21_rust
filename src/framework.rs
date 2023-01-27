use ndarray::{Array, Ix3, Ix2, Array3, Array2};
use rand::prelude::*;

// for binary array write
use ndarray_npy::{WriteNpyExt, WriteNpyError};
use std::fs::File;
use std::io::BufWriter;

pub const D: usize = 10;
pub const P: usize = 22;
pub const A: usize = 2;

#[derive(Clone)]
pub struct Algorithm {
    pub q: Array<f32, Ix3>,
    pub v: Array<f32, Ix2>,
    pub n: Array<i32, Ix3>,
}

impl Algorithm {

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

pub fn init_state() -> (usize, usize) {
    let dealer :Vec<usize> = (0..D).collect();
    let player: Vec<usize> = (0..P).collect();
    (
        *dealer.choose(&mut thread_rng()).unwrap(),
        *player.choose(&mut thread_rng()).unwrap()
    )
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

pub fn get_default() -> Algorithm {
    Algorithm {
        q: Array3::<f32>::zeros((D, P, A)),
        v: Array2::<f32>::zeros((D, P)),
        n: Array3::<i32>::zeros((D, P, A))
    }
}

pub fn yeet() -> Algorithm{
    println!("Yeeeeeeeeeeeeeeeeeeet");
    panic!()
}