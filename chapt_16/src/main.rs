use std::{sync::{mpsc, Arc, Mutex}, thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10{
            println!("Thread number {} created in spawn thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5{
        println!("Hi from main thread number {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    let vec = vec![1,2,3,4];

    let another =thread::spawn(move ||{
        println!("Here's a vector {:?}", vec);
    });

    another.join().unwrap();

    let (tx,rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    let (txn, rxn) = mpsc::channel();

    let txn1 = txn.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            txn1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("from"),
            String::from("you"),
        ];

        for val in vals{
            txn.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rxn {
        println!("Got: {}", received);
    }

    let m = Mutex::new(5); // mutex mutual exclusion

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
