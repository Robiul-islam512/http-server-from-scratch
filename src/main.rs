use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Read, Result, Write};
use std::fs::{File};

use response::content_type::ContentyType::ContentyType;
use response::response::response::{response};

mod request;
mod response;


fn main()->Result<()>{
    let litstener = TcpListener::bind("127.0.0.1:8080")?;
    
    for stream in litstener.incoming(){

        let mut stream = stream?;

        let buffer_values = buffer(&mut stream);

        let buffer_val = match buffer_values {
            Ok(buf)=>buf,
            Err(e)=>{
                eprintln!("Buffer Error: {}",e);
                ([0;4096],0)
            },
        };

        let buffer = buffer_val.0;
        let bytes = buffer_val.1;

        let data_info =  request::request::Request::request(buffer, bytes);


        match data_info {
            Ok(data)=>match data.get("method").map(|v| v.as_str()){
                Some("GET")=>{
                    let f = file("index.html");

                    let html_content = match f {
                        Ok(file)=>file,
                        Err(_)=>{
                            "<h1>Somthing Went Wrong<h1>".to_string()
                        }
                    };

                    let response = response(&ContentyType::TextHtml.as_str(), html_content);

                    stream.write_all(response.as_bytes())?;

                },
                Some("POST")=>{

                    let content = "robiul".to_string();

                    let response = response(&ContentyType::ApplicationJSON.as_str(), content);

                    println!("{}",response);

                    stream.write_all(response.as_bytes())?;
                },
                _=>{

                }
            },
            Err(e)=>{
                let message = e.to_string();
                let body = message.as_bytes();
                stream.write_all(body)?;
            }

        }

    }
    
    Ok(())
}

fn buffer(stream:&mut TcpStream)->Result<([u8;4096],usize)>{
    let mut buffer = [0;4096];
    let bytes = stream.read(&mut buffer)?;
    
    Ok((buffer,bytes))
}

fn file(file_name:&str)->Result<String>{
    
    let f = File::open(file_name)?;
    let read = BufReader::new(&f);

    let mut html_content = String::new();

    for content in read.lines(){
        let content = content?;
        html_content.push_str(&content);
    }

    Ok(html_content)
}
