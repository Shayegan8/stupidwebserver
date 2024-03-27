#![allow(unused)]

use std::net::{TcpListener, TcpStream};
use std::{env, fs};
use std::env::ArgsOs;
use std::io::{BufRead, BufReader, Write};

fn handle_conn(mut stream: TcpStream) {

    let buf_reader = BufReader::new(&mut stream);
    let request_line:String = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let status = "HTTP/1.1 200 OK";
        fs::read_dir("htmls");
        if let Ok(files) = fs::read_dir("htmls") {
            for file in files {
                if file.is_ok() {
                let path = file.unwrap().file_name();
                    let contents = fs::read_to_string(path).unwrap();
                    let length = contents.len();
                    let response = "{status}\r\nContent-Length: {length}\r\n\r\n{contents}";
                    stream.write_all(response.as_bytes()).unwrap();
                }
            }
        }
    } else {
        let status = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("htmls/404.html");
        let response = "{status}\r\nContent-Length: {length}\r\n\r\n{contents}";

        stream.write_all(response.as_bytes()).unwrap();
    }

    println!("REQ: {:#?}", request_line);
}

fn main() {

    let args: ArgsOs = env::args_os();
    let bind:TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("Listening on clients");

    for stream in bind.incoming() {
        let stream: TcpStream = stream.unwrap();
        handle_conn(stream);
    }
}