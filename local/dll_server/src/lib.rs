#![feature(pointer_byte_offsets)]
use core::ffi::c_void;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};
use winapi::um::libloaderapi::GetModuleHandleA;
use std::fs::OpenOptions;

extern crate directories;
use directories::BaseDirs;

use ue_types::*;

const MESSAGE_SIZE: usize = 1;
const OFFSET_FUNC_GETNAMES: isize = 0xf08e80;
const OFFSET_STRUCT_GOBJECTS: isize = 0x645FEC8;
const OFFSET_FUNC_GET_DISPLAY_NAME: isize = 0xF08E10;
// const OFFSET_FUNC_ULEVEL_GET_ACTORS: isize = 0x3F95240;

#[ctor::ctor]
fn ctor() {
    let base_addr = unsafe { GetModuleHandleA(std::ptr::null()) as *const c_void };
    let game_base = GameBase::generate(
        base_addr,
        OFFSET_STRUCT_GOBJECTS,
        OFFSET_FUNC_GETNAMES,
        OFFSET_FUNC_GET_DISPLAY_NAME
    );

    let game_base = GameBase::set_singleton(game_base);
    
    println!("Injected - Game Base: {:?}", GameBase::singleton());
    println!("UField: {:#01x?}", std::mem::size_of::<UField>());
    println!("UProperty: {:#01x?}", std::mem::size_of::<UProperty>());
    println!("UObject: {:#01x?}", std::mem::size_of::<UObject>());
    println!("AActor: {:#01x?}", std::mem::size_of::<AActor>());
    println!("FString: {:#01x?}", std::mem::size_of::<FString>());
    println!("FURL: {:#01x?}", std::mem::size_of::<FURL>());
    println!("ULevel: {:#01x?}", std::mem::size_of::<ULevel>());
    println!("Game Instance: {:#01x?}", game_base.game_instance().full_name());
    println!("Local Player Count: {:#01x?}", game_base.game_instance().local_players().len());
    println!("Local Player: {:?}", unsafe { *game_base.game_instance().local_players().at_index(0).unwrap() });
    println!("Local Player Name: {:?}", unsafe { (*game_base.game_instance().local_players().at_index(0).unwrap()).full_name() });
    println!("World: {:#01x?}", game_base.world());
    println!("Level: {:#01x?}", game_base.world().persistent_level());
    println!("Level Name: {:#01x?}", game_base.world().persistent_level().full_name());
    println!("Level (Actors Length): {:?}", game_base.world().persistent_level().actors().len());

    let actors = game_base.world().persistent_level().actors();
    println!("Actor[0] addr: {:#01x?}", actors.at_index(0).unwrap() );
    println!("Actor[0]: {:?}", unsafe { actors.at_index(0).unwrap().as_ref::<'static>().unwrap().full_name() } );


    println!("Starting TCP backdoor....");

    thread::spawn(|| {
        listen_for_connections();
    });

    println!("TCP Backdoor started!!");
}

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
                        writeln!(file, "GOBJECTS[{:?}]: {:?} ({:?})", i, obj.full_name(), obj.class().full_name()).unwrap();
                    }
                }
            }

            "get_root_objects" => {
                let game_base = GameBase::singleton();
                            
                println!("Local Player Count: {:#01x?}", game_base.game_instance().local_players().len());
                println!("Local Player: {:?}", unsafe { *game_base.game_instance().local_players().at_index(0).unwrap() });
                println!("Local Player Name: {:?}", unsafe { (*game_base.game_instance().local_players().at_index(0).unwrap()).full_name() });
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