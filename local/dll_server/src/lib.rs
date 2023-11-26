#![feature(pointer_byte_offsets)]
use core::ffi::c_void;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};
use winapi::um::libloaderapi::GetModuleHandleA;
use std::fs::OpenOptions;

extern crate directories;
use directories::BaseDirs;

use ue_types::GameBase;

const MESSAGE_SIZE: usize = 1;
const OFFSET_FUNC_GETNAMES: isize = 0xf08e80;
const OFFSET_STRUCT_GOBJECTS: isize = 0x645FEC8;
const OFFSET_FUNC_GET_DISPLAY_NAME: isize = 0xF08E10;
// const OFFSET_FUNC_ULEVEL_GET_ACTORS: isize = 0x3F95240;

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
    let log_path = format!("{}\\game-manager-log-server.log", BaseDirs::new().unwrap().home_dir().to_str().unwrap());
    let mut file = OpenOptions::new()
    .write(true)
    .truncate(true)
    .create(true)
    .open(log_path)
    .unwrap();

    loop {
        let message = read_message(&mut stream).expect("Could not parse message");

        match message.trim() {
            "" => (),
            
            "quit" => {
                println!("Client disconnected...");
                break;
            },

            "get_game_state" => {
                let g_objects = GameBase::singleton().gobjects();

                for i in 0..(g_objects.num_elements.to_native()-1) {
                    let item = g_objects.item_at_idx(i as usize);
                    let object = if item.is_some() { item.unwrap().object() } else { None };

                    if object.is_some() {
                        let obj = object.unwrap();
                        writeln!(file, "GOBJECTS[{:?}]: {:?} ({:?})", i, obj.get_full_name(), obj.class().config_name.to_string()).unwrap();
                    }
                }
            }

            "get_root_objects" => {
                let g_objects = GameBase::singleton().gobjects();

                for i in 0..(g_objects.num_elements.to_native()-1) {
                    let item = g_objects.item_at_idx(i as usize);
                    let object = if item.is_some() && item.unwrap().is_root_set() { item.unwrap().object() } else { None };

                    if object.is_some() {
                        let obj = object.unwrap();
                        
                        println!("ROOT_SET[{:?}]: {:?}", i, obj.get_full_name());
                    }
                }
            }

            "get_actors" => {
                let world = GameBase::singleton().world();
                let level = world.level();
                let actor = level.actors();
                
                println!("Actor: {:p} {:p} {:p}", world, level, actor);
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
    let base_addr = unsafe { GetModuleHandleA(std::ptr::null()) as *const c_void };
    let game_base = GameBase::generate(
        base_addr,
        OFFSET_STRUCT_GOBJECTS,
        OFFSET_FUNC_GETNAMES,
        OFFSET_FUNC_GET_DISPLAY_NAME
    );

    GameBase::set_singleton(game_base);
    
    println!("Injected - Game Base: {:?}", GameBase::singleton());

    println!("Starting TCP backdoor....");

    thread::spawn(|| {
        listen_for_connections();
    });

    println!("TCP Backdoor started!!");
}