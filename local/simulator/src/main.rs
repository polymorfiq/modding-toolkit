#![allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

fn handle_client(stream: TcpStream) {
    // ...
}

fn listen_for_connections() {
    println!("Waiting for TCP Connections...");
    let listener = TcpListener::bind("127.0.0.1:4951").expect("Could not open TCP Port");

    // accept connections and process them serially
    for stream in listener.incoming() {
        println!("New TCP Connection...");

        match stream {
            Ok(stream) => handle_client(stream),
            _ => ()
        }
        
    }
}

static GLOBAL_THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);
fn main() {
    println!("Starting TCP backdoor...");

    GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
    thread::spawn(|| {
        listen_for_connections();
        GLOBAL_THREAD_COUNT.fetch_sub(1, Ordering::SeqCst);
    });

    println!("TCP Backdoor started!");

    // Wait for other threads to finish.
    while GLOBAL_THREAD_COUNT.load(Ordering::SeqCst) != 0 {
        thread::sleep(Duration::from_millis(1)); 
    }
}