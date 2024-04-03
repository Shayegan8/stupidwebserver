#![allow(unused)]

use local_ip_address::local_ip;
use std::env::Args;
use std::error::Error;
use std::ffi::OsString;
use std::fmt::Debug;
use std::io::Error as Er;
use std::io::ErrorKind as Kind;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path;
use std::path::PathBuf;
use std::process::exit;
use std::{env, fmt, fs};

fn handle_conn(
    mut stream: TcpStream,
    vec: &Vec<OsString>,
    mut address: &String,
) -> Result<(), std::io::Error> {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    buf_reader.read_line(&mut request_line)?;
    let request_lines: Vec<&str> = request_line.lines().collect();

    let mut is_writted = false;
    for value in vec {
        let astar = &<OsString as Clone>::clone(&value).into_string().unwrap();
        if request_lines[0].contains(astar) {
            println!("contains!");
            let status = "HTTP/1.1 200 OK";
            let contents = fs::read_to_string(format!("htmls/{}", astar))?;
            let length = contents.len();
            let response = format!(
                "{status}\r\nContent-Length: {length}\r\n\r\n{contents}",
                status = status,
                length = length,
                contents = contents
            );
            stream.write_all(response.as_bytes())?;
            is_writted = true;
            break;
        } else {
            continue;
        }

        if (is_writted == false) {
            println!("writing 404");
            let status = "HTTP/1.1 404 NOT FOUND";
            let contents = fs::read_to_string("htmls/404.html")?;
            let length = contents.len();
            let response = format!(
                "{status}\r\nContent-Length: {length}\r\n\r\n{contents}",
                status = status,
                length = length,
                contents = contents
            );
            stream.write_all(response.as_bytes())?;
        }
    }

    println!("REQ: {:#?}", request_line);
    Ok(())
}
fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut vec: Vec<OsString> = vec![];
    let path = PathBuf::from("htmls");
    for string in path.read_dir().unwrap() {
        let mrpath = string.unwrap().file_name();
        vec.push(mrpath);
    }

    if env::args().count() == 1 {
        eprintln!("Port is not valid");
        println!("{}", local_ip().unwrap().to_string());
        std::process::exit(1);
    }

    let mut address = String::new();
    address.push_str(&format!(
        "{address}:",
        address = local_ip().unwrap().to_string()
    ));
    address.push_str(&*args[1]);

    let bind: Result<TcpListener, Er> = TcpListener::bind(&address);

    match bind {
        Ok(ref bind) => println!("Listening on clients"),
        Err(e) => {
            println!("Port is already in use or not exist");
            exit(1);
        }
    }

    let bind = bind.unwrap_or_else(|msg| {
        eprintln!("Can't bind listener");
        exit(1);
    });

    for stream in bind.incoming() {
        match stream {
            Ok(stream) => {
                handle_conn(stream, &vec, &address).unwrap();
            }
            Err(err) => {
                eprint!("{}", err);
            }
        }
    }
}
