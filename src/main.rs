use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{self, BufRead, BufReader, Read, Result, Write};
use std::fs::{File};

use request::request::request::request;
use router::get::get_request::{
    get_route_file_path,
    get
};

use router::post::post_request::post;

use errors::errors::errors::HttpErrors;
use components::login::login::error_req;

use parsing::parsing::parsing::{parsing,Request};
// use parsing::parsing::parsing



mod request;
mod response;
mod errors;
mod router;
mod components;
mod parsing;

fn main()->Result<()>{
    let litstener = TcpListener::bind("127.0.0.1:8080")?;
    
    for stream in litstener.incoming(){

        let stream = tcp_stream(stream);

        let mut stream = match stream {
            Ok(strm)=>strm,
            Err(e)=>{
                eprintln!("ERROR: {}",e);
                continue;
            }
        };

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

        let request_parse =  parsing(buffer,bytes);

        let method = request_parse.request_line.method;

        match method.as_str() {
            "GET"=>{
                let route_path = request_parse.request_line.path;
                let query_params = if !request_parse.request_line.query_params.is_empty(){
                    request_parse.request_line.query_params
                }else{
                    HashMap::new()
                };

                let file_path = get_route_file_path(&route_path);

                let content = match file(&file_path){
                        Ok(cnt) =>cnt,
                        Err(_)=>{
                            "<h1>Invalid Path</h1>".to_string()
                        }
                    };

                let _ = stream.write_all(get(&content).as_bytes());
            },
            "POST"=>{
                let url = request_parse.request_line.path;
                let params = request_parse.request_line.query_params;
                let body = match request_parse.body {
                    Some(body)=>body,
                    None=>HashMap::new()
                };

                let res = match post(url,body,params){
                        Ok(res)=>res,
                        Err(e)=>{
                            eprintln!("{}",e);
                            format!("{}",e)
                        }
                    };

                    println!("response: {}",res); 
                    let _ = stream.write_all(res.as_bytes());
            },
            _=>{
                
            }
        }      

    }
    
    Ok(())
}



pub fn tcp_stream(stream:io::Result<TcpStream>)->std::result::Result<TcpStream,HttpErrors>{
    let er = error_req("server error".to_string(), "Internal server error".to_string());
    match stream {
        Ok(strm)=>Ok(strm),
        Err(_)=>{
            return Err(er);
        }
    }
}

fn write(stream:&mut TcpStream,server_response:String)->Result<()>{

    stream.write_all(server_response.as_bytes())?;

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
