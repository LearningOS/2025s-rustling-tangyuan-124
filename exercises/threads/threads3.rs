// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.



use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

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

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> Vec<thread::JoinHandle<()>> {
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);

    let tx1 = tx.clone();
    let handle1 = thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            if let Err(e) = tx1.send(*val) {
                eprintln!("Error sending value: {}", e);
                break;
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    let tx2 = tx.clone();
    let handle2 = thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            if let Err(e) = tx2.send(*val) {
                eprintln!("Error sending value: {}", e);
                break;
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    vec![handle1, handle2]
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    let handles = send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    // 等待所有发送线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);
}
