#![allow(unused_imports)]
#![allow(unused)]
use simple_endian::*;
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;
use std::io::{Error, ErrorKind, Read, Write};
use core::ffi::c_void;
use retour::static_detour;
use winapi::um::libloaderapi::GetModuleHandleA;

static mut BASE_ADDR: usize = 0; // Probably going to be 0x7FF674A80000
const MESSAGE_SIZE: usize = 1;
const OFFSET_LIST_PLAYERS: usize = 0x3084F20;
const OFFSET_STRUCT_GOBJECTS: usize = 0x645FEC8;
const OFFSET_FUNC_GET_GAME_STATE: usize = 0xB05A20;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct FUObjectArray {
    obj_first_gc_idx: u32le,
    obj_last_non_gc_idx: u32le,
    max_objects_not_considered_by_gc: u32le,
    open_for_disregard_for_gc: u8,
    _padding: [u8; 3],
    objects_ptr: u64,
    pre_allocated_objects_ptr: u64,
    max_elements: u32le,
    num_elements: u32le,
    max_chunks: u32le,
    num_chunks: u32le,
    various_data: [u8; 990]
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
    loop {
        let message = read_message(&mut stream).expect("Could not parse message");

        match message.trim() {
            "" => (),
            
            "quit" => {
                println!("Client disconnected...");
                break;
            },

            "get_game_state" => {
                let ptr = offset_addr(OFFSET_STRUCT_GOBJECTS) as *const FUObjectArray;
                let g_obj_ptr: &FUObjectArray = unsafe { std::mem::transmute(ptr) };
                // let get_game_state: extern "C" fn(*mut c_void) -> *mut c_void = unsafe { std::mem::transmute(ptr) };
                // let resp = (get_game_state)(world_ptr);
                // let numResp = unsafe { std::mem::transmute::<*mut c_void, u64>(resp) };
                let g_objects: FUObjectArray = *g_obj_ptr;
                println!("Data: {:#01x} {:?}", ptr as u64, g_objects);
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

fn offset_addr(offset: usize) -> *mut c_void {
    unsafe { (BASE_ADDR + offset) as *mut c_void }
}

#[ctor::ctor]
fn ctor() {
    unsafe { BASE_ADDR = GetModuleHandleA(std::ptr::null()) as usize };
    println!("INJECTED AT BASE ADDRESS: {:#01x}", unsafe { BASE_ADDR });

    println!("Starting TCP backdoor....");

    thread::spawn(|| {
        listen_for_connections();
    });

    println!("TCP Backdoor started!!");
}