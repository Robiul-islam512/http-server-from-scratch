use std::net::{TcpListener};
use std::io::{BufRead, BufReader, Read, Result, Write};
use std::fs::{File};

use response::content_type::ContentyType::ContentyType;
use response::response::Response::response;

mod request;
mod response;


fn main()->Result<()>{
    let litstener = TcpListener::bind("127.0.0.1:8080")?;
    
    for stream in litstener.incoming(){
        let mut buffer = [0;4096];
        let mut stream = stream?;

        let bytes =  stream.read(&mut buffer)?;

        let data_info =  request::request::Request::request(buffer, bytes);


        match data_info {
            Ok(data)=>match data.get("method").map(|v| v.as_str()){
                Some("GET")=>{
                    let f = File::open("index.html")?;
                    let read = BufReader::new(f);

                    let mut html_content = String::new();

                    for content in read.lines(){
                        let content = content?;
                        html_content.push_str(&content);
                    }

                    let response = response(&ContentyType::TextHtml.as_str(), html_content);

                    stream.write_all(response.as_bytes())?;

                },
                Some("POST")=>{

                },
                _=>{

                }
            },
            Err(e)=>{
                println!("{:?}",e);
                let message = e.to_string();
                let body = message.as_bytes();
                stream.write_all(body)?;
            }

        }

    }
    
    Ok(())
}


