mod thread_pool;

use std::{
    fs,
    io::{Read, Result, Write},
    net::{TcpListener, TcpStream},
};

use crate::thread_pool::ThreadPool;

const ONE_KILOBYTE: usize = 1024;

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0; ONE_KILOBYTE];
    stream.read_exact(&mut buffer)?;

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "static/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "static/404.html")
    };

    let body = fs::read_to_string(filename)?;

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        body.len(),
        body
    );

    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}

pub fn run() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream?;

        pool.execute(|| {
            if let Err(error) = handle_connection(stream) {
                println!("An error occurred: {}", error);
            }
        })
    }

    Ok(())
}
