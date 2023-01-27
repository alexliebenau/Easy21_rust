use std::iter::Sum;
use ndarray::{ArcArray, Array2, Array3};
use rayon::iter::plumbing::UnindexedConsumer;
use crate::framework::{Algorithm, D, P, A};
use rayon::prelude::*;

pub fn return_instance(iterations: u64) -> Algorithm {
    Algorithm {
        q: ArcArray::from(Array3::<f32>::zeros((D, P, A))).to_owned(),
        v: ArcArray::from(Array2::<f32>::zeros((D, P))).to_owned(),
        n: ArcArray::from(Array3::<i32>::zeros((D, P, A))).to_owned()
    }
        .get_q_rayon(iterations)
        .get_v()

    // let tst = [return_instance(100000); 100000];
}

impl Algorithm {
    fn get_q_rayon(mut self, iterations: u64) -> Self {

        self = (0..iterations).into_par_iter()
            //.par_iter_mut()
            .map(|x| self.get_q_single(x))
            .sum();
        self
    }
}

// impl Iterator for Algorithm {
//    type Item = Algorithm;
//
//     fn next(&mut self) -> Option<Self::Item> {
//        if
//        }
//     }
// }

struct AlgIntoIter {
    alg: Algorithm,
    index: usize
}

impl IntoIterator for Algorithm {
    type Item = Algorithm;
    type IntoIter = AlgIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        AlgIntoIter {
            alg: self,
            index: 0
        }
    }
}

impl Iterator for AlgIntoIter {
    fn next(&mut self) -> Option<>
}

impl Sum for Algorithm {
    fn sum<I: Iterator>(iter: I) -> Self {
        todo!()
    }
}

impl ParallelIterator for AlgIntoIter {
    type Item = Algorithm;

    fn sum<S>(self) -> S where S: Send + Sum<Self::Item> + Sum<S> {
        todo!()
    }

    fn drive_unindexed<C>(self, consumer: C) -> rayon::iter::plumbing::Result where C: UnindexedConsumer<Self::Item> {
        todo!()
    }
}