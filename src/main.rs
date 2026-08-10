use std::net::{TcpListener,TcpStream};
use std::io::{Read, Result, Write};
use std::str::Bytes;
use chrono::{DateTime, Local};

mod request;
mod response;

fn main()->Result<()>{
    let litstener = TcpListener::bind("127.0.0.1:8080")?;
    
    for stream in litstener.incoming(){
        let mut buffer = [0;4096];
        let mut stream = stream?;

        let bytes =  stream.read(&mut buffer)?;

        let info =  request::request::Request::request(buffer, bytes);

        let response = response::response::Response::response();


        stream.write_all(response.as_bytes())?;

        
       
    }
    
    Ok(())
}


