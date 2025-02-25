use std::{
    fs,
    net::{ TcpListener, TcpStream },
    io::{ BufReader, prelude::* }
};

fn main() {

    match TcpListener::bind("127.0.0.1:7878") {

        Ok(listener) => {
            
            for stream in listener.incoming() {

                request_handler(stream);

            };

        },

        Err(err) => println!("{:#?}", err)

    }

}

fn request_handler(mut stream: Result<TcpStream, std::io::Error>) {

    match stream {

        Ok(mut stream) => {

            let buf_reader = BufReader::new(&stream);

            let request_line = buf_reader
                .lines()
                .next()
                .unwrap()
                .unwrap();

            let (status_line, text) = if request_line == "GET / HTTP/1.1" {
                ("HTTP:/1.1 200 OK", "<h1>Home Page</h1>")
            } else if request_line == "GET /about HTTP/1.1" {
                ("HTTP:/1.1 200 OK", "<h1>About Page</h1>")
            } else {
                ("HTTP:/1.1 404 PAGE NOT FOUND", "<h1>Page not found!</h1>")
            };

            let content = String::from(text);
            let length = content.len();

            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

            stream.write_all(response.as_bytes()).unwrap();

        },

        Err(err) => println!("{:#?}", err)

    }

}
