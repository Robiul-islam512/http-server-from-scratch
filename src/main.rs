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

use parsing::parsing::parsing::parsing;



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

        parsing(buffer,bytes);

        let data_info = request(buffer, bytes);

        match data_info {
            Ok(data)=>match data.get("method").map(|v| v.as_str()){
                Some("GET")=>{
                    
                    let url_path = match data.get("url"){
                        Some(path)=>path.to_string(),
                        None=>"/".to_string(),
                    };

                    let file_path = get_route_file_path(&url_path);

                    let content = match file(&file_path){
                        Ok(cnt) =>cnt,
                        Err(_)=>{
                            "<h1>Invalid Path</h1>".to_string()
                        }
                    };

                    let _ = stream.write_all(get(&content).as_bytes());
                    
                },
                Some("POST")=>{

                    let res = match post(&data, buffer, bytes){
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
                    eprintln!("Does not match with any method");
                }
            },
            Err(e)=>{
                let message = e.to_string();
                let _ = write(&mut stream, message);
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
