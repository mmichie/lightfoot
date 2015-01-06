#![feature(macro_rules, phase, slicing_syntax)]

use std::thread::Thread;
use std::io::Timer;
use std::rand;
use std::time::Duration;

static N_THREADS: int = 1000;

fn main() {
    let _ = (0..N_THREADS).map( |i| Thread::spawn(move || {
        let t = rand::random::<u8>();
        println!("this is thread number {}, sleeping for {}", i, t);
        let mut timer = Timer::new().unwrap();
        timer.sleep(Duration::milliseconds(t as i64));
        println!("this is thread number {}, finished sleeping for {}", i, t);
    })).collect::<Vec<_>>();
}
