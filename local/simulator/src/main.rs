#![allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;
use std::io::{Error, ErrorKind, Read, Write};

const MESSAGE_SIZE: usize = 1;

fn read_message(stream: &mut TcpStream) -> Result<String, std::string::FromUtf8Error> {
    // Store all the bytes for our received String
    let mut received: Vec<u8> = vec![];

    // Array with a fixed size
    let mut rx_bytes = [0u8; MESSAGE_SIZE];
    loop {
        // Read from the current data in the TcpStream
        let bytes_read = stream.read(&mut rx_bytes).expect("Failed to read stream");
        let recv_len = received.len();

        if rx_bytes == "\n".as_bytes() {
            break;
        }

        // However many bytes we read, extend the `received` string bytes
        received.extend_from_slice(&rx_bytes[..bytes_read]);

        // If we didn't fill the array
        // stop reading because there's no more data (we hope!)
        if bytes_read < MESSAGE_SIZE {
            break;
        }
    }

    String::from_utf8(received)
}

fn handle_client(mut stream: TcpStream) {
    loop {
        let message = read_message(&mut stream).expect("Could not parse message");

        match message.trim() {
            "quit" => {
                println!("Client disconnected...");
                break;
            }

            "get_players" => {
                stream.write(b"42\n").expect("Tried to write to TCP Stream");
            }

            msg => println!("Unknown Message: {msg}")
        }
    }
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
        };
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