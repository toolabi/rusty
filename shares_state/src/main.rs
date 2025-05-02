
// shared state cocnurency is like a one way data flow
// the lock is a data structure that keeps track of which thread has exclusive access to the data
// mutex rules:
// you have to acquire lock before you have access to the data
// you release the lock when you're done with the data
use std::sync::{Arc, Mutex};
use std::thread;
fn main() {


    let v = Arc::new(Mutex::new(0));
    let mut handles =Vec::new();

    for _ in 0 .. 10 {
        let counter = Arc::clone(&v);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num +=1;

        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap()
    }


    println!("{:?}", *v.lock().unwrap());

}
