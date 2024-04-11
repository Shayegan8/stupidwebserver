mod papijoy;

use crossterm::{cursor, style, terminal, ExecutableCommand, QueueableCommand};
use jemalloc_ctl::{epoch, stats};
use papijoy::papijoy as papi;

use colored::Colorize;
use local_ip_address::local_ip;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::ffi::OsString;
use std::fs::OpenOptions;
use std::io::{stdin, Error as Er, LineWriter};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::{env, fs, thread};

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn handle_conn(mut stream: TcpStream, vec: &Vec<OsString>) -> Result<(), std::io::Error> {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    buf_reader.read_line(&mut request_line)?;
    let request_lines: Vec<&str> = request_line.lines().collect();

    let mut is_writted = false;
    for value in vec {
        let (_new_astar1, new_astar2) = request_lines[0].split_at(5);
        let nnew_astar = new_astar2.split(" ").nth(0).unwrap();
        let vaue = value.to_str().unwrap();
        if nnew_astar == vaue {
            let status = "HTTP/1.1 200 OK";
            let contents = fs::read_to_string(format!("htmls/{}", vaue))?;
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
    }
    if is_writted == false {
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

    let file = OpenOptions::new()
        .append(true)
        .write(true)
        .create(true)
        .open("logs/latest.log")
        .unwrap();

    let mut file = LineWriter::new(file);
    let str = format!("{}\n", stream.peer_addr().unwrap().to_string());
    file.write(str.as_bytes()).unwrap();
    file.flush().unwrap();
    println!("REQ: {:#?}\n> ", request_line);
    Ok(())
}

fn mem(atbol: Arc<AtomicBool>) {
    thread::spawn(move || {
        let mut stdout = std::io::stdout();
        stdout.queue(cursor::Hide).unwrap();
        while atbol.load(Ordering::SeqCst) {
            epoch::advance().unwrap();
            stdout.queue(cursor::SavePosition).unwrap();
            let jemallac = stats::allocated::read().unwrap().to_string().green();
            stdout
                .write_all(format!("{}", jemallac).as_bytes())
                .unwrap();
            stdout.queue(cursor::SetCursorStyle::BlinkingBlock).unwrap();
            stdout.queue(cursor::RestorePosition).unwrap();
            stdout.flush().unwrap();
            thread::sleep(Duration::from_secs(1));

            stdout.queue(cursor::RestorePosition).unwrap();
            stdout
                .queue(terminal::Clear(terminal::ClearType::FromCursorDown))
                .unwrap();
        }
        stdout.queue(cursor::Show).unwrap();
    })
    .join()
    .unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut vec: Vec<OsString> = vec![];
    let path = PathBuf::from("htmls");
    for string in path.read_dir().unwrap_or_else(|_m| {
        panic!(
            "{}",
            "Please make a htmls directory in here and add 404.html in it".on_red()
        )
    }) {
        let mrpath = string
            .unwrap_or_else(|_m| {
                panic!("nothing >:(");
            })
            .file_name();
        vec.push(mrpath);
    }

    let mut address = String::new();
    address.push_str(&format!(
        "{address}:",
        address = local_ip().unwrap().to_string()
    ));

    if env::args().count() == 1 {
        println!("{}", "Port is not valid we use property".on_red());
        address.push_str(&papi::get_property(
            "port",
            &String::from("25565"),
            "config.dcnf",
        ));
    } else if env::args().count() == 2 {
        address.push_str(&*args[1]);
    }
    let property = papi::get_property("addressbind", &address, "config.dcnf");

    println!(
        "{}{}\n{}{}",
        "Welcome to the ".bright_green(),
        "Stupidwebserver".bright_yellow(),
        "A webserver will be binded in ".bright_green(),
        property.on_red()
    );
    println!(
        "{} {} {}",
        "do".bright_green(),
        "help".on_red(),
        " to get commands".bright_green()
    );
    
    let atbol = Arc::new(AtomicBool::new(true));
    let atbol_clone = atbol.clone();

    ctrlc::set_handler(move || {
        atbol_clone.store(false, Ordering::SeqCst);
    })
    .unwrap();
    thread::spawn(move || {
        
        loop {
            print!("> ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let tstrin = input.trim();

            if tstrin.eq("help") {
                println!(
                    "{}{}\n{}{}\n{}{}",
                    "help".yellow(),
                    " - this command".green(),
                    "shutdown".yellow(),
                    " - it shutdowns webserver".green(),
                    "showlog".yellow(),
                    " - it shows logs/latest.log lines".green()
                );
            } else if tstrin.eq("shutdown") {
                println!("{}", "Bye!".green());
                exit(0);
            } else if tstrin.eq("showlog") {
                println!("{}\n", "logs/latest.log".on_purple());
                fs::read_to_string("logs/latest.log").iter().for_each(|x| {
                    println!("{}", x.green());
                });
            } else if tstrin.eq("memory") {
                mem(atbol.clone());
            } else {
                println!(
                    "{}{}{}",
                    "Command not found! do ".green(),
                    "help ".yellow(),
                    "to get commands".green()
                )
            }
        }
    });

    let bind: Result<TcpListener, Er> = TcpListener::bind(&property);

    match bind {
        Ok(ref _bind) => println!(
            "{}{}",
            "BOUNDED!".bright_yellow(),
            " Listening on clients...".bright_green()
        ),
        Err(_e) => {
            println!("{}", "port is already inuse or not exist".on_red());
            exit(1);
        }
    }

    let bind = bind.unwrap_or_else(|_msg| {
        eprintln!("Can't bind listener");
        exit(1);
    });

    for stream in bind.incoming() {
        match stream {
            Ok(stream) => {
                handle_conn(stream, &vec).unwrap();
            }
            Err(err) => {
                eprint!("{}", err);
            }
        }
    }
}
