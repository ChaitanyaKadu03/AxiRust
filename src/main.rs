use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, prelude::*};
use std::fs;

fn main() {

    let listner = TcpListener::bind("localhost:7878").unwrap();

    for stream in listner.incoming() {
        
        let stream = stream.unwrap();

        handle_connection(stream);   
    }
    
}

fn handle_connection(mut stream: TcpStream){
    
    let buf_reader = BufReader::new(&stream);
    
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, contents) =  if request_line == "GET / HTTP/1.1" {
        
        ("HTTP/1.1 200 OK", fs::read_to_string("home.html").unwrap())

    } else {

        ("HTTP/1.1 404 NOT FOUND", fs::read_to_string("404.html").unwrap())

    };

    let length = contents.len();

    let response = format!(
            "{status_line}\r\nContent-Length : {length}\r\n\r\n{contents}"
        );

    stream.write_all(response.as_bytes()).unwrap();

}
