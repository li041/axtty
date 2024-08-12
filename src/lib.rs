//! This crate provides a tty system.
#![cfg_attr(all(not(test), not(doc)), no_std)]

//mod port;
mod buffer;
mod tty;
mod ldisc;
mod driver;
mod port;