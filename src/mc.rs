mod single;

// use crate::{framework};
use crate::framework;


pub fn run(i: i32) -> framework::Algorithm {

   single::instance(i)
}