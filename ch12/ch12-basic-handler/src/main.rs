#![cfg(not(windows))]

use std::time::{Duration};
use std::thread::{sleep};
use libc::{SIGTERM, SIGUSR1};

static mut SHUT_DONW: bool = false;

fn main() {
    register_signal_handlers();

    let delay = Duration::from_secs(1);

    for i in 1_usize.. {
        println!("{}", i);
        unsafe {
            if SHUT_DONW {
                println!("*");
                return;
            }
        }
        sleep(delay);

        let signal = if i > 2 {
            SIGTERM
        } else {
            SIGUSR1
        };

        unsafe {
            libc::raise(signal);
        }
    }
    unreachable!();
}

fn register_signal_handlers() {
    unsafe {
        libc::signal(SIGTERM, handle_sigtterm as usize);
        libc::signal(SIGUSR1, handle_sigusr1 as usize);
    }
}

#[allow(dead_code)]
fn handle_sigtterm(_signal: i32) {
    register_signal_handlers();
    println!("SIGTTERM");

    unsafe {
        SHUT_DONW = true;
    }
}

#[allow(dead_code)]
fn handle_sigusr1(_signal: i32) {
    register_signal_handlers();
    println!("SIGUSR1")
}
