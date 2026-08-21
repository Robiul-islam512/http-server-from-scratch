use std::net::{TcpListener, TcpStream};
use std::io::{self, BufRead, BufReader, Read, Result, Write};
use std::fs::{File};

use response::content_type::content_type::ContentyType;
use response::response::response::{response};
use request::request_error::request_error::HttpError;
use request::request::request::request;
use request::data_decoding::data_decoding::data_extraction;


mod request;
mod response;


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

        let data_info = request(buffer, bytes);

        match data_info {
            Ok(data)=>match data.get("method").map(|v| v.as_str()){
                Some("GET")=>{
                    let f = file("index.html");

                    let html_content = match f {
                        Ok(file)=>file,
                        Err(_)=>{
                            "<h1>Server Error<h1>".to_string()
                        }
                    };

                    let response = response(&ContentyType::TextHtml.as_str(),html_content.clone());

                    // println!("{}",response);
                    println!("{:?}",data);
                    stream.write_all(response.as_bytes());  
                    stream.write_all(&html_content.as_bytes());

                },
                Some("POST")=>{

                    let user_data = match data.get("body"){
                        Some(val)=>{
                            let data = data_extraction(val);
                            data
                        },
                        None=>{
                           String::new()
                        }
                    };

                    println!("{:?}",user_data);
                    println!("{:?}",data);

                    let content = user_data;

                    let response = response(&ContentyType::ApplicationJSON.as_str(), content);

                    
                    stream.write_all(response.as_bytes());
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



fn tcp_stream(stream:io::Result<TcpStream>)->std::result::Result<TcpStream,HttpError>{
    match stream {
        Ok(strm)=>Ok(strm),
        Err(e)=>{
            return Err(HttpError::ServerError(e.to_string()));
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
