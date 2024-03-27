#![allow(unused)]

use std::net::{TcpListener, TcpStream};
use std::{env, fs};
use std::env::ArgsOs;
use std::fmt::Error;
use std::io::{BufRead, BufReader, Write};
use std::io::Error as Er;
use std::io::ErrorKind as Kind;

fn handle_conn(mut stream: TcpStream) -> Result<(), Er> {

    let buf_reader = BufReader::new(&mut stream);
    let request_line:String = match buf_reader.lines().next() {
        Some(Ok((line))) => line,
        _ => return Err(Er::new(Kind::InvalidData, "")),
    };

    if request_line == "GET / HTTP/1.1" {
        let status = "HTTP/1.1 200 OK";
        let files = fs::read_dir("htmls").expect("Cant read >:(");
            for file in files {
                if let Ok(file) = file {
                    let path = file.file_name();
                    let contents = fs::read_to_string(path).expect("Problem with reading lines");
                    let length = contents.len();
                    let response =
                        format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}", status = status, length = length, contents = contents);

                    stream.write_all(response.as_bytes())?;
                }
            }
    } else {
        let status = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("htmls/404.html").expect("Problem with reading lines");
        let length = contents.len();
        let response =
            format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}", status = status, length = length, contents = contents);

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
