// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

// 

use std::sync::mpsc;
use std::sync::{Arc,Mutex};
use std::thread;
use std::time::Duration;


// 问题分析
// 主线程没有等待子线程完成：

// 原来的程序中，send_tx 函数会启动两个子线程发送数据，但主线程并没有等待这些线程完成。主线程直接开始接收通道中的数据，并在接收完成后立即结束。
// 因为子线程是异步执行的，如果主线程在子线程还未完成发送之前就接收完所有数据并退出，可能会导致未发送的数据丢失。这会导致接收到的数据数量不正确，进而引发断言失败（assert_eq!(total_received, queue_length)）。
// tx（发送端）的所有权问题：

// 在原始代码中，send_tx 函数只是简单地将发送端 tx 移动到两个子线程中。在第一个线程结束后，tx 的所有权可能会被丢弃，从而导致第二个线程无法继续使用这个发送端。
// 为了解决这个问题，需要将 tx 包装在 Arc 和 Mutex 中，使其可以被多个线程安全地共享。
// 共享数据的竞争条件：

// 多个线程同时访问和修改共享数据时，可能会发生竞争条件。在原始代码中，这个问题还未显现，但在一些复杂的场景下，如果多个线程尝试同时发送数据或访问共享数据结构，而没有适当的同步机制（例如使用 Mutex），会引发数据竞争问题。

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);
    let tx = Arc::new(Mutex::new(tx));

    let tx1 = tx.clone();
    let tx2 = tx.clone();
    let handle1  = thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx1.lock().unwrap().send(*val).unwrap(); 
            thread::sleep(Duration::from_secs(1));
        }
    });

    let handle2 = thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx2.lock().unwrap().send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}
