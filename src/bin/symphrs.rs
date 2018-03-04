extern crate symphrs;

use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs::File;
use std::thread;
use std::time::Duration;
use symphrs::thread::ThreadPool;
use symphrs::thread::JobResult;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1666").unwrap();
    let pool = ThreadPool::new(4).unwrap();
    let mut counter = 0;

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream)
        });

        counter += 1;

        if counter >= 2 {
            break;
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> JobResult {
    let mut buffer = [0; 2048];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]).to_string();


    let home_page = b"GET / HTTP/1.1\r\n";
    let sleep_page = b"GET /sleep HTTP/1.1\r\n";

    let (response_headers, template_name) = if buffer.starts_with(home_page) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep_page) {
        thread::sleep(Duration::from_secs(8));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 Not Found\r\n\r\n", "404.html")
    };

    let mut file = File::open("templates/".to_string() + template_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", response_headers, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    JobResult::new(request, response)
}
