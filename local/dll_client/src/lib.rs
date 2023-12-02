#![feature(pointer_byte_offsets)]
use core::ffi::c_void;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};
use winapi::um::libloaderapi::GetModuleHandleA;
// use std::fs::OpenOptions;
// extern crate directories;
// use directories::BaseDirs;

use ue_types::*;
use utils::{debug, warning};

const MESSAGE_SIZE: usize = 1;
const OFFSET_FUNC_GETNAMES: isize = 0x10E94B0;
const OFFSET_STRUCT_GOBJECTS: isize = 0x753EC50;
const OFFSET_FUNC_GET_DISPLAY_NAME: isize = 0x10E9440;

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
    // let log_path = format!("{}\\game-manager-log-client.log", BaseDirs::new().unwrap().home_dir().to_str().unwrap());
    // let mut file = OpenOptions::new()
    // .write(true)
    // .truncate(true)
    // .create(true)
    // .open(log_path)
    // .unwrap();

    loop {
        let message = read_message(&mut stream).expect("Could not parse message");

        match message.trim() {
            "" => (),
            
            "quit" => {
                debug!("Client disconnected...");
                break;
            },

            "get_game_state" => {
                let g_objects = GameBase::singleton().gobjects();

                for i in 0..(g_objects.num_elements.to_native()-1) {
                    let item = g_objects.item_at_idx(i as usize);
                    let object = if item.is_some() { item.unwrap().object::<UObject>() } else { None };

                    if object.is_some() {
                        let obj = object.unwrap();
                        debug!("GOBJECTS[{:?}]: {:?}", i, obj.full_name());
                    }
                }
            }

            "get_players" => {
                stream.write(b"42\n").expect("Tried to write to TCP Stream");
            },

            msg => warning!("Unknown Message: {msg}")
        }
    }
}

fn listen_for_connections() {
    debug!("Starting TCP Listener...");
    let listener = TcpListener::bind("0.0.0.0:4951").expect("Could not open TCP Port");
    debug!("Waiting for TCP Connections...");

    // accept connections and process them serially
    for stream in listener.incoming() {
        debug!("New TCP Connection...");

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
    
    debug!("Injected - Game Base: {:?}", GameBase::singleton());

    debug!("Starting TCP backdoor....");


    let game_base = GameBase::singleton();
    let g_objects = game_base.gobjects();

    for i in 0..(g_objects.num_elements.to_native()-1) {
        let item = g_objects.item_at_idx(i as usize);
        let object = if item.is_some() { item.unwrap().object::<UObject>() } else { None };

        if object.is_some() &&  game_base.get_offset_from_addr(object.unwrap().virtual_funcs()) == 0x5ab1450 {
            let obj: &UObject = object.unwrap();
            debug!("GOBJECTS[{:?}]: {:?} ({:?})", i, obj.full_name(), obj.class().full_name());
        }
    }

    thread::spawn(|| {
        listen_for_connections();
    });

    debug!("TCP Backdoor started!!");
}