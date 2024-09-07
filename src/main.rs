use std::{io::{BufRead, BufReader}, net::{TcpListener, TcpStream}, prelude::*};

fn main() {
    let listener = TcpListener::bind("localhost:3000").unwrap();
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        handle_connection(_stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buff_reader = BufReader::new(stream);
    let http_request : Vec<_> = buff_reader.lines().map(|result| result.unwrap()).take_while(|line| !line.is_empty()).collect();
    println!("Stream Data : ");
    println!("{http_request:#?}");
}