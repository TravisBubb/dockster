use std::{
    fs::{exists, remove_file},
    io::Read,
    os::unix::net::{UnixListener, UnixStream},
    process::exit,
    thread,
};

fn main() {
    const SOCKET_PATH: &str = "/tmp/docksterd.socket";

    if exists(SOCKET_PATH).expect("Error checking if socket exists") {
        if let Err(error) = remove_file(SOCKET_PATH) {
            println!("Error removing socket {}: {}", SOCKET_PATH, error);
            exit(1);
        }
    }

    let listener = match UnixListener::bind(SOCKET_PATH) {
        Ok(listener) => listener,
        Err(error) => {
            println!("Error binding socket: {}", error);
            exit(1);
        }
    };

    loop {
        match listener.accept() {
            Ok((sock, _)) => {
                thread::spawn(|| handle_client(sock));
            }
            Err(error) => {
                println!("Failed to accept incoming connection: {}", error);
            }
        }
    }
}

fn handle_client(mut stream: UnixStream) {
    println!("handling connection for client");

    let mut buf = String::new();
    match stream.read_to_string(&mut buf) {
        Ok(num_bytes) => println!("Read {} bytes: {}", num_bytes, buf),
        Err(error) => println!("Error reading from socket: {}", error),
    }
}
