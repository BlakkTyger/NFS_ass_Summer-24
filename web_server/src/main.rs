use std::{
    fs,
    io::{prelude::*, BufReader},
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
    let buf_reader = BufReader::new(&mut stream);
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        _ => {
            let response = format!("HTTP/1.1 400 Bad Request\r\n\r\n");
            stream.write_all(response.as_bytes()).unwrap();
            println!("HTTP/1.1 400 Bad Request");
            return;
        }
    };

    let (status_line, filename) = match request_line.as_str(){
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /only-for-pros HTTP/1.1" => ("HTTP/1.1 403 Forbidden", "403.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(_) => {
            if status_line == "HTTP/1.1 404 NOT FOUND" {
                println!("{}", status_line); // Log the status line
            }
            let error_page = match status_line {
                "HTTP/1.1 404 NOT FOUND" => include_str!("404.html"),
                "HTTP/1.1 403 Forbidden" => 
                {
                    println!("{}", status_line); // Log the status line
                    include_str!("403.html")
                }
                _ => {
                    let error_page = include_str!("500.html");
                    let response = format!(
                        "HTTP/1.1 500 INTERNAL SERVER ERROR\r\nContent-Length: {}\r\n\r\n{}",
                        error_page.len(),
                        error_page
                    );
                    stream.write_all(response.as_bytes()).unwrap();
                    println!("HTTP/1.1 500 INTERNAL SERVER ERROR");
                    return;
                }
            };
            let response = format!(
                "{}\r\nContent-Length: {}\r\n\r\n{}",
                status_line,
                error_page.len(),
                error_page
            );
            stream.write_all(response.as_bytes()).unwrap();
            return;
        }
    };

    println!("{}", status_line); // Log the status line
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
}
