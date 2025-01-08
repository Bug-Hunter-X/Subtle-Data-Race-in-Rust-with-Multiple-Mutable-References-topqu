use std::sync::{Arc, Mutex};

fn main() {
    let x = Arc::new(Mutex::new(5));
    let y = Arc::clone(&x);
    let z = Arc::clone(&x);
    
    let mut y_guard = y.lock().unwrap();
    *y_guard = 10; 
    
    let mut z_guard = z.lock().unwrap();
    *z_guard = 100; 
    
    println!("The final value of x: {}", *x.lock().unwrap());
}