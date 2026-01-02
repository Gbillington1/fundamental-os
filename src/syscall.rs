// src/syscall.rs
// implements all the syscalls

use crate::{ printk, printk_u64 };
use crate::exceptions::TrapFrame;

pub(crate) fn handle_syscall(tf: &mut TrapFrame, _iss: u64) {
    // get syscall number 
    let id = tf.x[8];

    printk("Syscall "); printk_u64(id); printk(" triggered but not implemented.\n"); 
    
    // for now, return -1 to indicate that nothing is implemented
    tf.x[0] = !0;
}
