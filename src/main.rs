use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{self, BufRead, BufReader, Read, Result, Write};
use std::fs::{File};

use response::content_type::content_type::ContentyType;
use request::request_error::request_error::HttpError;
use request::request::request::request;
use request::data_decoding::data_decoding::data_extraction;
// use request::router::router::{get_router,post_router};
use router::get::get_request::{get_route_file_path,get
};
use router::post::post_request::post;


mod request;
mod response;
mod errors;
mod router;


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
                            format!("{}",e)
                        }
                    };

                    println!("response: {}",res); 
                    
                    
                    stream.write_all(res.as_bytes());
                    
                    
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

fn url_path(map_data:&HashMap<String,String>)->String{
    match map_data.get("url"){
        Some(path)=>path.to_string(),
        None=>"/".to_string(),
    }                  
}

// fn get_response(url_path:String,path:String,stream:&mut TcpStream){
//     let response =  match get_router("GET",url_path,path){
//         Ok(values)=>values,
//         Err(e)=>{
//             eprintln!("Route Matching Error: {}",e);
//             ("".to_string(),"".to_string())
//         }
//     };

//     stream.write_all(response.0.as_bytes());
//     stream.write_all(response.1.as_bytes());

// }

pub fn tcp_stream(stream:io::Result<TcpStream>)->std::result::Result<TcpStream,HttpError>{
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
