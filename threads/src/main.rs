use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    // drop(v); // v was moved to another thread so we can not
    // use it in main thread, not if we dont move it here again

    handle.join().unwrap();

    println!("--------------------------------------------");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("asdfasfd");
    println!("{}", rx.recv().unwrap());
    println!("{}", rx.recv().unwrap());
    for received in &rx {
        println!("Got: {}", received);
    }
    println!("{:?}", rx);
    println!("--------------------------------------------");
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    println!("--------------------------------------------");
    println!("The following cauese a deadlock, cause we have two threads waiting for mutexes to be released which never happens");
    println!("As they're are waiting for one to release another they get stuck");
    let x = Arc::new(Mutex::new(0));
    let y = Arc::new(Mutex::new(0));
    let mut handles: Vec<JoinHandle<()>> = vec![];

    for _ in 0..2 {}
    {
        let x = x.clone();
        let y = y.clone();
        let handle = thread::spawn(move || {
            let mut x = x.lock().unwrap();
            *x += 4;
            thread::sleep(Duration::from_secs(3));
            let mut y = y.lock().unwrap(); // we are waiting for other thread to
                                           // release y
            println!("This line shouldn't be printed cause of the deadlock")
        });
        handles.push(handle);
    }
    {
        let x = x.clone();
        let y = y.clone();
        let handle = thread::spawn(move || {
            let mut y = y.lock().unwrap();
            *y += 4;
            let mut x = x.lock().unwrap(); // we are waiting for other thread to
                                           // release the x
            println!("This line shouldn't be printed cause of the deadlock")
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap(); // this makes the program to wait till threads are done
                                // but they will never be done waiting for values to be unlocked
    }
    println!("End of the program");
}
