
//! 24.8.7 没有异步工作队列, 中断处理函数直接调用n_tty_receive_buf

extern crate alloc;
use crate::{buffer::RingBuffer, tty::Tty};
use alloc::{sync::Weak, boxed::Box};


pub struct TtyPort {
    // back pointer to struct tty_struct 
    tty: Weak<Tty>,
    inner: Box<dyn TtyPortOps>,
    flip_buf: RingBuffer,
}

pub trait TtyPortOps {
    /// tty port operations
    fn activate(&self);
    fn shutdown(&self) { unimplemented!(); }
    /// tty port client operations
    /// only support Uart now, read and write byte by byte
    fn receive_buf(&mut self);
    fn write_wakeup(&mut self);
}