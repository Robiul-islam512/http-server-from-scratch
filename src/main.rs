use std::net::{TcpListener};
use std::io::{BufRead, BufReader, Read, Result, Write};
use serde_json::Value;
use std::fs::{File, read, write};
use std::error::Error;

use response::content_type::ContentyType::ContentyType;
use response::response::Response::response;

use crate::request::method::Method::MethodType;

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


        // let method = match data_info{
        //     Ok(data)=> match data.get("method").map(|v| v.as_str()){
        //         Some("GET")=>MethodType::GET,
        //         Some("POST")=>MethodType::POST,
        //         _=>MethodType::GET,
        //     }
        //     Err(e)=>{
        //         stream.write_all(e)
        //         // stream.write_all)
        //     }
        // };


        // if &method.as_str() == "GET"{

        //     let f = File::open("index.html")?;

        //     let read =  BufReader::new(f);

        //     let mut html_file = String::new();

        //     for line in read.lines(){
        //         let line = line?;
        //         html_file.push_str(&line);
        //     }

        //     let text_html_content = ContentyType::TextHtml.as_str();
        //     let application_json = ContentyType::ApplicationJSON.as_str();

        //     let response = response(&text_html_content,html_file);
        

        //     stream.write_all(response.as_bytes())?;
        // }


       
    }
    
    Ok(())
}


