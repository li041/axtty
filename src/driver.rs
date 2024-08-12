#![allow(unused)]
extern crate alloc;
use alloc::{collections::BTreeMap, sync::Arc, string::String};

use spinlock::SpinNoIrq;

use crate::tty::Tty;



pub struct TtyDriver {
    ttys: SpinNoIrq<BTreeMap<i32, Arc<Tty>>>,
    index: i32,
    name: String, 
}