use std::ffi::c_void;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::panic;
use std::io::{Read, Write};
use game_base::*;

mod match_state;
use match_state::*;

static MOD_NAME: &'static str = "match_tracker";

#[no_mangle]
fn mod_main_sync(base_addr: *const c_void) {
    unsafe { game_base::OFFSETS = game_base::offsets_server::get_offsets() };
    GameBase::initialize(MOD_NAME, base_addr);

    println!("Starting Match Tracker....");

    thread::spawn(|| {
        listen_for_connections();
    });

    println!("Match Tracker started!!");
}

const MESSAGE_SIZE: usize = 1;

fn handle_client(mut stream: TcpStream) {
    loop {
        let message = read_message(&mut stream);
        if !message.is_ok() { break };

        let message = message.unwrap();
        match message.trim() {
            "" => (),
            
            "quit" => {
                println!("Client disconnected...");
                break;
            },

            "get_key_addresses" => {
                stream.write(format!("WORLD PROXY: {:p}\n", WorldProxy::proxy().expect("No World Proxy...")).as_bytes()).expect("Tried to write to TCP Stream");
                stream.write(format!("WORLD: {:p}\n", WorldProxy::world().expect("No World...")).as_bytes()).expect("Tried to write to TCP Stream");
                stream.write(format!("LEVEL: {:p}\n", WorldProxy::level().expect("No Level...")).as_bytes()).expect("Tried to write to TCP Stream");
                stream.write(format!("GAME INSTANCE: {:p}\n", GameInstance::instance().expect("No Game Instance...").base().unwrap()).as_bytes()).expect("Tried to write to TCP Stream");
                break;
            },

            "get_players" => {
                let match_state = get_match_state();
                let json_str = serde_json::to_string(&match_state).unwrap();

                stream.write(json_str.as_bytes()).expect("Tried to write to TCP Stream");
                stream.write(b"\n").expect("Tried to write to TCP Stream");
                break;
            },

            msg => {
                println!("Unknown Message: {msg}");
                break;
            }
        }
    }
}


fn read_message(stream: &mut TcpStream) -> Result<String, std::string::FromUtf8Error> {
    // Store all the bytes for our received String
    let mut received: Vec<u8> = vec![];

    // Array with a fixed size
    let mut rx_bytes = [0u8; MESSAGE_SIZE];
    loop {
        // Read from the current data in the TcpStream
        let bytes_read = stream.read(&mut rx_bytes);
        if !bytes_read.is_ok() { break };
        let bytes_read = bytes_read.unwrap();

        // let recv_len = received.len();

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
                    panic::catch_unwind(|| {
                        handle_client(stream);
                        println!("Ending TCP Connection...");
                    })
                });
            },

            _ => ()
        };
    }
}