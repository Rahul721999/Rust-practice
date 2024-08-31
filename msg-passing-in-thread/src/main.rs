// TODO: 1. Create a channel to send and recieve channel. 2. Create two thread, 3. One thread will send msg through channel and another one will recieve.

use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

fn main() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    // Thread 1: Sending messages
    let sender_thread = thread::spawn(move || {
        for i in 0..10 {
            println!("Sending: {i}");
            tx.send(i).expect("Failed to send msg through thread one");
        }
    });

    // Thread 2: Receiving messages
    let receiver_thread = thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            println!("Got msg: {msg}");
        }
        println!("Channel closed, no more messages to receive.");
    });

    // Wait for both threads to finish
    sender_thread.join().expect("Thread one got panicked");
    receiver_thread.join().expect("Thread two got panicked");
}
