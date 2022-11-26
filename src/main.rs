#![feature(type_name_of_val)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]

extern crate crosslogger_server;

use crate::runner::{run};

mod runner;

fn main() {
    run();
}
