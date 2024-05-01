use multi_threaded_server::ThreadPool;
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    // Starting listening to our 7878 port
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // creating our threads which will execute jobs
    let pool = ThreadPool::new(4);
    // iterating throuhg our requests
    for stream in listener.incoming() {
        // if one request fail our program will panic
        let stream = stream.unwrap();
        // sending jobs to our thread pool
        pool.execute(|| handle_connection(stream));
    }
    println!("Sutting donw..");
}
// Handling the routes requested
fn handle_connection(mut stream: TcpStream) {
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
    stream.flush().unwrap();
}
