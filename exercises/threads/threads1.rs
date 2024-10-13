// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.

// 

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || { // 让每个线程都在主线程结束之前结束，否则主线程结束后，子线程也会结束，主线程持有子线程的所有权，子线程结束后，子线程的所有权也会结束，所以子线程的所有权会被释放，子线程也会结束
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis()
        }));
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?
        results.push(handle.join().unwrap());
// handle.join().unwrap(): 等待线程结束，并获取其返回值。
// join() 方法会阻塞当前线程，直到目标线程结束。
// unwrap() 用于处理可能的错误，如果线程在执行过程中遇到 panic，将会导致程序 panic。
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread W{} took {}ms", i, result);
    }
}
