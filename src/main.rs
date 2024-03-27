#![allow(unused)]

use std::net::{TcpListener, TcpStream};
use std::{env, fs};
use std::env::ArgsOs;
use std::io::{BufRead, BufReader, Write};

fn handle_conn(mut stream: TcpStream) {
    let reader:BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    let req: Vec<_> = reader.lines().map(|x| x.unwrap())
        .take_while(|line| !line.is_empty()).collect();

    let req_line = req.lines().next().unwrap().unwrap();

    if req_line == "GET / HTTP/1.1" {
        let status = "HTTP/1.1 200 OK";
        fs::read_dir("htmls");
        for file in fs::read_dir("htmls") {
            let file = file?;
            let path = file.path();
            if path.is_file() {
                let contents = fs::read_to_string(path).unwrap();
                let length = contents.len();
                let response = "{status}\r\nContent-Length: {length}\r\n\r\n{contents}";
                stream.write_all(response.as_bytes()).unwrap();
            }
        }
    } else {
        let status = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("htmls/404.html");
        let response = "{status}\r\nContent-Length: {length}\r\n\r\n{contents}";

        stream.write_all(response.as_bytes()).unwrap();
    }

    println!("REQ: {:#?}", req);
}

fn main() -> u32 {

    let args: ArgsOs = env::args_os();
    let bind:TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("Listening on clients");

    for stream in bind.incoming() {
        let stream: TcpStream = stream.unwrap();
        handle_conn(stream);
    }

    0
}