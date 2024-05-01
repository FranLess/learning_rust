use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // -------------- (VERSION 1)
    // let buf_reader = BufReader::new(&mut stream);
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    // // let response = "HTTP/1.1 200 OK\r\n\r\n";
    // // stream.write_all(response.as_bytes()).unwrap()

    // let status_line = "HTTP/1.1 200 OK";
    // let contents = fs::read_to_string("hello.html").unwrap();
    // let length = contents.len();
    // let response = format!("{status_line}\r\nContent_Length: {length}\r\n\r\n{contents}");
    // stream.write_all(response.as_bytes()).unwrap();

    // -------------------- (VERSION 2)

    // let buf_reader = BufReader::new(&mut stream);
    // let request_line = buf_reader.lines().next().unwrap().unwrap();
    // if request_line == "GET / HTTP/1.1" {
    //     let status_line = "HTTP/1.1 200 OK";
    //     let contents = fs::read_to_string("hello.html").unwrap();
    //     let length = contents.len();
    //     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    //     stream.write_all(response.as_bytes()).unwrap();
    // } else if request_line == "GET /test HTTP/1.1" {
    //     let status_line = "HTTP/1.1 200 OK";
    //     let contents = fs::read_to_string("test.html").unwrap();
    //     let length = contents.len();
    //     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    //     stream.write_all(response.as_bytes()).unwrap();
    // } else {
    //     let status_line = "HTTP/1.1 404 NOT FOUND";
    //     let contents = fs::read_to_string("404.html").unwrap();
    //     let length = contents.len();
    //     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    //     stream.write_all(response.as_bytes()).unwrap();
    // }

    // ------------------(REFACTORING VERSION 2)
    // NOTE: This is so cool, we've just made some kind of routing system
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /test HTTP/1.1" => ("HTTP/1.1 200 OK", "test.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
