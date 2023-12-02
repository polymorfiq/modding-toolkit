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
use injection_utils::InjectionBase;
use utils::{debug, warning};

// Game Instance VFTable: 0x48AD730 (0x7FF67932D730)
// Game Engine VFTable: 0x4dfc3e8 (0x7FF67987C3E8)

const MESSAGE_SIZE: usize = 1;
const OFFSET_FUNC_GETNAMES: isize = 0xf08e80;
const OFFSET_STRUCT_GOBJECTS: isize = 0x645FEC8;
// const OFFSET_STRUCT_GAME_INSTANCE_VFTABLE: isize = 0x48AD730;
// const OFFSET_STRUCT_ENGINE: isize = 0x4DFC3E8;
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

    utils::init();

    let game_base = GameBase::set_singleton(game_base);
    debug!("Injected - Game Base: {:p}", GameBase::singleton());
    debug!("World Name: {:#01x?}", game_base.world().full_name());

    // let interesting_item = game_base.gobjects().item_at_idx(60901).expect("Failed to find Interesting");
    // let interesting_obj = interesting_item.object::<UGameEngine>().expect("Unable to unwrap Interesting object");
    // println!("Interesting: {:#01x?}", interesting_obj);
    
    debug!("Game Instance Name: {:#01x?}", game_base.game_instance().full_name());
    debug!("World Context: {:p}", game_base.game_instance().world_context());
    debug!("World Context World: {:?}", game_base.game_instance().world_context().world());
    debug!("World ADDR 2: {:p}", game_base.game_instance().world_context().world());
    debug!("World Name 2: {:?}", game_base.game_instance().world_context().world().full_name());
    debug!("Game Instance: {:#01x?}", game_base.game_instance().full_name());
    debug!("Local Player Count: {:#01x?}", game_base.game_instance().local_players().len());
    debug!("Level (Actors Length): {:?}", game_base.world().persistent_level().actors().len());

    debug!("Starting TCP backdoor....");

    thread::spawn(|| {
        listen_for_connections();
    });

    debug!("TCP Backdoor started!!");
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
                debug!("Client disconnected...");
                break;
            },

            "override_god_cheat" => {
                unsafe { InjectionBase::override_god(); }
            },

            "get_game_state" => {
                let g_objects = GameBase::singleton().gobjects();

                for i in 0..(g_objects.num_elements.to_native()-1) {
                    let item = g_objects.item_at_idx(i as usize);
                    let object = if item.is_some() { item.unwrap().object() } else { None };

                    if object.is_some() {
                        let obj: &UObject = object.unwrap();
                        writeln!(file, "GOBJECTS[{:?}]: {:?} ({:?})", i, obj.full_name(), obj.class().full_name()).unwrap();
                    }
                }
            },

            "get_actors" => {
                let actors = GameBase::singleton().world().persistent_level().actors();
                for i in 0..(actors.len()) {
                    let actor = actors.at_index(i);
                    let actor_ref = if actor.is_some() { unsafe { (*actor.unwrap()).as_ref() } } else { None };

                    if actor_ref.is_some() {
                        debug!("Actor[{}]: {:?}", i, actor_ref.unwrap().full_name() );
                    }
                }
            },

            "get_players" => {
                stream.write(b"42\n").expect("Tried to write to TCP Stream");
            },

            msg => warning!("Unknown Message: {msg}")
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