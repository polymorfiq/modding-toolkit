#![allow(unused_imports)]
#![allow(unused)]
use core::ffi::c_void;
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;
use std::io::{Error, ErrorKind, Read, Write};
use retour::static_detour;
use winapi::um::libloaderapi::GetModuleHandleA;

pub mod game;

const MESSAGE_SIZE: usize = 1;

fn read_message(stream: &mut TcpStream) -> Result<String, std::string::FromUtf8Error> {
    // Store all the bytes for our received String
    let mut received: Vec<u8> = vec![];

    // Array with a fixed size
    let mut rx_bytes = [0u8; MESSAGE_SIZE];
    loop {
        // Read from the current data in the TcpStream
        let bytes_read = stream.read(&mut rx_bytes).expect("Failed to read stream");

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
            "" => (),
            
            "quit" => {
                println!("Client disconnected...");
                break;
            },

            "get_game_state" => {
                let g_objects = game::get_gobjects();
                let g_names = game::get_gnames();
                
                println!("GOBJECTS: {:?}", g_objects.objects_array);
                println!("GNAMES: {:?}", g_names);
            }

            "get_players" => {
                stream.write(b"42\n").expect("Tried to write to TCP Stream");
            },

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
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream)
                });
            },

            _ => ()
        };
    }
}

#[ctor::ctor]
fn ctor() {
    game::init();
    println!("INJECTED AT BASE ADDRESS: {:#01x}", game::get_base_addr() as usize);
    println!("FOUND GNAMES ADDRESS: {:#01x}", game::get_gnames_addr() as usize);

    println!("Starting TCP backdoor....");

    thread::spawn(|| {
        listen_for_connections();
    });

    println!("TCP Backdoor started!!");
}