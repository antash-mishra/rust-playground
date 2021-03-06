use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::Mutex;

fn main() {

    let v = vec![1,2,3];

    let handle = thread::spawn( || {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }          
    });

    let handle2 = thread::spawn(move || {
        println!("Here's the vector {:?}", v);
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }


    handle.join().unwrap();
    handle2.join().unwrap();

    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    for received in rx {
        println!("Got: {}", received);
    }

    let (tx1, rx1) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx1);
    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("motherfuckers"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    std::thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("fuckers"),
            String::from("in"),
            String::from("channel"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for received in rx1 {
        println!("got: {}", received);
    }

    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}   