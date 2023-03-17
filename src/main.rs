use std::{
    env,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: smol_server <ip> <port>");
        std::process::exit(1);
    }

    let ip = &args[1];
    let port = &args[2];

    let listener = TcpListener::bind(format!("{}:{}", ip, port)).unwrap();

    println!("smol_server listening on: {}:{}", ip, port);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    let msg = r"  _________________
< i am a smolserver >
  -----------------
         \   ^__^ 
          \  (oo)\_______
             (__)\       )\/\
                 ||----w |
                 ||     ||";

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", msg);
    stream.write_all(response.as_bytes()).unwrap();
}
