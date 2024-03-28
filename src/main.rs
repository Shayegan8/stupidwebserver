#![allow(unused)]

use std::net::{TcpListener, TcpStream};
use std::{env, fmt, fs};
use std::env::Args;
use std::error::Error;
use std::fmt::Debug;
use std::io::{BufRead, BufReader, Write};
use std::io::Error as Er;
use std::io::ErrorKind as Kind;
use local_ip_address::local_ip;

fn handle_conn(mut stream: TcpStream) -> Result<(), Er> {

    let buf_reader = BufReader::new(&mut stream);
    let request_line:String = match buf_reader.lines().next() {
        Some(Ok((line))) => line,
        _ => return Err(Er::new(Kind::InvalidData, "Cant read from data")),
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

    let mut args: Vec<String> = env::args().collect();


    if env::args().count() == 1 {
        eprintln!("Port is not valid");
        std::process::exit(1);
    }

    let mut address = String::new();
    address.push_str(&format!("{address}:", address = local_ip().unwrap().to_string()));
    address.push_str(&*args[1]);

    let bind: Result< TcpListener, Er> = TcpListener::bind(address);

    match bind {
        Ok(ref bind) => println!("Listening on clients"),
        Err(e) => {
            println!("Port is already in use");
            std::process::exit(1);
        },
    }

    let bind = bind.unwrap();

        for stream in bind.incoming().into_iter() {
            let stream: TcpStream = stream.unwrap();
            handle_conn(stream);
        }
}
