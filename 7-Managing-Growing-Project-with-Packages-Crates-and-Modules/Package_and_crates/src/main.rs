// use std::fmt;
// use std::io;
// use std::{fmt, io};
// use crate::garden::vegetables::Asparagen;
use garden::vegetables::Asparagus;
// use std::io;
// use std::io::Write;
// use std::io::*; // glob operator
use std::io::{self, Write};

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growin {:?}!", plant)
}
