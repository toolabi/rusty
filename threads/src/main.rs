use core::time;
use std::thread;
use std::sync::mpsc::{self, channel};
use std::time::Duration;

fn main() {
    // concurrent programming => different part of your programs execute independently
    // parallel programming => different part of your programs execute at the same time
    println!("Hello, world!");
    // threads is the feature that runs diffrent programs independly 
    // race condition:threads accesing data and resources in an inconsistent order
    // deadlocks: both threads are waitnig for the resourses the other thread has => waiting indefenitly 
    // hard to manage bugs => because its hard to recreate the sitution and conditions

    // green thread model is not one-to-one it's n*m
    // when the main thread end the spawn thread stops


    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    let v = 0;
    // if we want to use variables in thread we have to give their ownership to closure
    // since the compiler does not know how long will the closure run and if the is not dropped

    let the_big_t = thread::spawn(move ||{
        // let m = String::from("you gotta see thisssss.");
        let string_v = [
            String::from("at"),
            String::from("least"),
            String::from("as"),
            String::from("deep"),
            String::from("as"),
            String::from("the"),
            String::from("pacific"),
            String::from("ocean"),
        ];
        for i in string_v{
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));

        }
        // for i in 0 .. 10 {
        //     println!("thread spwan: {:?}", i);
        //     println!("thread v: {:?}", v);
        //     thread::sleep(time::Duration::from_millis(1));
        // }
    });




    let the_small_t = thread::spawn(move ||{
        // let m = String::from("you gotta see thisssss.");
        let string_v = [
            String::from("i"),
            String::from("wanna"),
            String::from("be"),
            String::from("yours"),
        ];
        for i in string_v{
            tx1.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));

        }

    });

    for m in rx {
        println!("{:?}", m);
    }

    // the main thread will wait for the thread to stop executing


    // for i in 0 .. 5 {
    //     println!("thread main: {:?}", i);
    //     thread::sleep(time::Duration::from_millis(1));
    // }

    // if we dont join threads the big t wont run completely


    // use channels to pass message between threads
    // transmitter => where you send
    // receiver => where you receive


}
