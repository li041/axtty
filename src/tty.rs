use crate::{driver::TtyDriver, ldisc::TtyLdisc, port::TtyPort};

extern crate alloc;

use alloc::{string::String, sync::{Weak, Arc}};

pub struct Tty {
    index: i32,  
    name: String, 
    port: TtyPort,
    // driver: Weak<TtyDriver>,
    ldisc: Arc<TtyLdisc>,
}

