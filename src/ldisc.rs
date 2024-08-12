extern crate alloc;

use crate::{buffer::TtyBuffer, tty::Tty};
use alloc::sync::Weak;
use spinlock::SpinNoIrq; 

pub struct TtyLdisc {
    // back pointer to tty_struct 
    tty: Weak<Tty>,
    /// chars that can be read by kernel
    read_buf: TtyBuffer,
    /// chars being echoed on the screen
    echo_buf: TtyBuffer,
}

pub trait TtyLdiscOps {
    // The following routines are called from below

    /// read into tty->port->flip_buf
    fn receive_buf();

    // The following routines are called from above
    fn read_byte();
    fn write_byte();
}