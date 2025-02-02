use std::error::Error;

// use std::{fs, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};
// use web_server::ThreadPool;
use tokio::{
    fs, io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::{TcpListener, TcpStream}
};

#[tokio::main]
async fn main() -> Result< (),Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            handle_connection(stream).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next_line().await.unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // let contents = fs::read_to_string(filename).unwrap();
    let contents = fs::read_to_string(filename).await.unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).await.unwrap();
}
