use std::sync::{Arc, Mutex};
use std::thread;

fn main(){
    read_by_multiple_threads();
    write_by_multiple_threads();
}
fn read_by_multiple_threads() {
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = Vec::new();
    for i in 0..3 {
        let data_cloned = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let sum: i32 = data_cloned.iter().sum();
            println!("Thread: {i}: sum = {sum}");
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("Main thread can still use it: len = {}", data.len());
}

fn write_by_multiple_threads(){
    let counter = Arc::new(Mutex::new(0i64));
    let mut handles = Vec::new();
    for _ in 0..8{
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move||{
            for _ in 0..100{
                *c.lock().unwrap()+=1;
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("Counter = {}", *counter.lock().unwrap());
}