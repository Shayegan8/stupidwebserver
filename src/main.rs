#![allow(unused)]

use std::net::{TcpListener, TcpStream};
use std::{env, fs};
use std::env::ArgsOs;
use std::io::{BufRead, BufReader, Write};

fn handle_conn(mut stream: TcpStream) -> Result<(), std::io::Error> {
    let mut buf_reader = BufReader::new(&mut stream);
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid request line")),
    };

    if request_line == "GET / HTTP/1.1" {
        let status = "HTTP/1.1 200 OK";
        let files = fs::read_dir("htmls").expect("Error reading directory");
        for file in files {
            if let Ok(file) = file {
                let path = file.file_name();
                let contents = fs::read_to_string(path).expect("Error reading file contents");
                let length = contents.len();
                let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}", status = status, length = length, contents = contents);
                stream.write_all(response.as_bytes())?;
            }
        }
    } else {
        let status = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("htmls/404.html").expect("Error reading 404 file");
        let length = contents.len();
        let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}", status = status, length = length, contents = contents);
        stream.write_all(response.as_bytes())?;
    }

    println!("REQ: {:#?}", request_line);
    Ok(())
}
fn main() {

    let args: ArgsOs = env::args_os();
    let bind:TcpListener = TcpListener::bind("127.0.0.1:21039").unwrap();

    println!("Listening on clients");

    for stream in bind.incoming() {
        let stream: TcpStream = stream.unwrap();
        handle_conn(stream);
    }
}
