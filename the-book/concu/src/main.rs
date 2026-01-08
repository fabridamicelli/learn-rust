use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
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

    for received in rx {
        println!("Got: {received}");
    }
}

// fn main() {
//     let (tx, rx) = mpsc::channel();
//
//     thread::spawn(move || {
//         let val = String::from("hi");
//         tx.send(val).unwrap();
//     });
//
//     let out = rx.recv().unwrap();
//     println!("{}", out)
// }

// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {i} from the spawned thread!");
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
//
//     for i in 1..5 {
//         println!("hi number {i} from the main thread!");
//         thread::sleep(Duration::from_millis(1));
//     }
//
//     handle.join().unwrap();
// }

// fn main() {
//     let v = vec![1, 2, 3];
//
//     let handle = thread::spawn(move || {
//         println!("Here's a vector: {v:?}");
//     });
//
//     handle.join().unwrap();
// }
