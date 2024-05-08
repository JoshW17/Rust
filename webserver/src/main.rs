// My first webserver in Rust!

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration
};

fn main(){
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap(); // bind to localhost port 8080
    
    for stream in listener.incoming() {
        let stream = stream.unwrap(); // Our new TCP stream
        thread::spawn(|| {            // Now we can handle multiple requests concurrently but we don't have a limit on the # of threads              
            handle_connection(stream); // Now actually handle a HTTP request and pass it our TCP stream
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_method_header = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_method_header == "GET / HTTP/1.1" || request_method_header == "GET /hello HTTP/1.1" {
        ("HTTP/1.1 200 OK", "static/hello.html")
    } else if  request_method_header == "GET /sleep HTTP/1.1" {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "static/sleep.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "static/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
