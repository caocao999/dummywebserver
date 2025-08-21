use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::env::{self,args};

fn main() {
    if args().count() >= 3 {
        eprintln!("引数が変ですよ。");
        std::process::exit(1);
    }

    let args :Vec<String> = env::args().collect();
    let mut port = "80";
    if let Some(n) = args.get(1) {
        port = n;
    } 
   
    let tcp_addr = format!("127.0.0.1:{}",port);
    println!("PORT = {}",tcp_addr);

    let listener =  match TcpListener::bind(&tcp_addr){
        Ok(listen) => listen,
        Err(e) => {
            eprintln!("ポートの取得に失敗 {}:{}",tcp_addr,e);
            std::process::exit(1);
        }
    };

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream,port);
    }
}

fn handle_connection(mut stream:TcpStream,port:&str)
{
    let mut buffer= [0;1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}",String::from_utf8_lossy(&buffer[..]));

    let content = format!("<!DOCTYPE html><html lang='ja'><head><title>Document</title></head>\
        <body><h1>Success at PORT = {}</h1></body>\
        </html>",port);
    let body = content.as_bytes();
    let headers = format!(
        "HTTP/1.1 200 OK\r\n\
        Content-Type: text/html; charset=utf-8\r\n\
        Content-Length: {}\r\n\
        Connection: close\r\n\r\n",
        body.len()
    );
    if let Err(e) = stream.write_all(headers.as_bytes()) { eprintln!("write hdr: {e}"); return; }
    if let Err(e) = stream.write_all(body) { eprintln!("write body: {e}"); return; }
}
