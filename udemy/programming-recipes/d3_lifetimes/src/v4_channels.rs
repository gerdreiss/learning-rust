use core::time::Duration;
use std::sync::{Arc, Mutex};

fn main() {
    //with_arc();
    with_channels();
}

pub fn with_arc() {
    let m = Arc::new(Mutex::new(String::from("moving")));
    let m2 = m.clone();

    std::thread::spawn(move || {
        println!("This is the new channel");
        let mut s2 = m2.lock().unwrap();
        s2.push_str(" on the new thread");
        println!("m = {}", s2);
    });

    std::thread::sleep(Duration::from_millis(1000));
    println!("This is the initial channel");
    let s = m.lock().unwrap();
    println!("now m = {}", s);
}

pub fn with_channels() {
    let (ch_s, ch_r) = std::sync::mpsc::channel::<Box<dyn Fn(&mut String) + Send>>();
    let (done_s, done_r) = std::sync::mpsc::channel::<()>();

    std::thread::spawn(move || {
        let mut hidden = String::new();
        loop {
            match ch_r.recv() {
                Ok(f) => {
                    f(&mut hidden);
                    println!("hidden = {}", hidden);
                }
                Err(e) => {
                    println!("Done with {}", e);
                    done_s.send(()).unwrap();
                    return;
                }
            }
        }
    });

    ch_s.send(Box::new(|s: &mut String| {
        s.push_str("hello");
    }))
    .unwrap();

    let ch_2 = ch_s.clone();

    ch_2.send(Box::new(|s: &mut String| {
        s.push_str(" world");
    }))
    .unwrap();

    drop(ch_s);
    drop(ch_2);

    done_r.recv().unwrap();

    //std::thread::sleep(Duration::from_millis(1000));
}
