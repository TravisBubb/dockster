use std::{
    io::{Read, Write},
    net::Shutdown,
    os::unix::net::UnixStream,
    process::exit,
};

mod container;

fn main() {
    const SOCKET_PATH: &str = "/tmp/docksterd.socket";

    // attempt to connect to the Unix domain socket
    let mut conn = match UnixStream::connect(SOCKET_PATH) {
        Ok(conn) => conn,
        Err(error) => {
            println!("Failed to connect to daemon: {}", error);
            exit(1);
        }
    };

    let message = "hello there".as_bytes();

    // attempt to write a message to the server
    if let Err(error) = conn.write_all(message) {
        println!("Failed to write message to socket: {}", error);
        exit(1);
    }

    // close the write connection and tell the server we are finished writing
    if let Err(error) = conn.shutdown(Shutdown::Write) {
        println!(
            "An error occurred attempting to shutdown write connection: {}",
            error
        );
        exit(1);
    }

    let mut buf = String::new();
    match conn.read_to_string(&mut buf) {
        Ok(num_bytes) => println!("Read {} bytes: {}", num_bytes, buf),
        Err(error) => println!("Error reading from socket: {}", error),
    }
}
