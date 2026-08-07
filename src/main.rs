use std::fs::File;
use std::net::{TcpListener,TcpStream};
use std::io::{BufRead, BufReader, Read, Result, Write};
use serde_json::json;

#[derive(Debug)]
enum Method{
    GET,
    POST,
    PUT,
    DELETE,
    HEAD, 
}

#[derive(Debug)]
struct URL<'a>{
    url:&'a str,
}

#[derive(Debug)]
struct RequestLine<'a>{
    method:Method,
    url:URL<'a>,
    version:&'a str,
}

#[derive(Debug)]
struct RequestHeaders<'a>{
    header:Vec<&'a str>
}

#[derive(Debug)]
struct EntityBody<'a>{
    body:&'a str,
}

#[derive(Debug)]
struct RequestFormat<'a>{
    request_line:RequestLine<'a>,
    header_lines:Vec<RequestHeaders<'a>>,
    blank_line:&'a str,
    body:EntityBody<'a>,
}


fn main()->Result<()>{
    let litstener = TcpListener::bind("127.0.0.1:8080")?;

    let data_file = File::open("messages.txt")?;

    let mut buffer = [0;4096];

    let data = "hello world";

    let response = format!("HTTP/1.1 200 OK\r\n
Content-Type: text/plain\r\n
Content-Length: 23\r\n
{}",data);

    // let name = "robiu\r\rl";
    // println!("{}",name);

   
    for stream in litstener.incoming(){
        
        let mut stream = stream?;

        let _ = stream.read(&mut buffer);

        let mut requested_data = String::new();

        let mut data_start = 0;

        let mut sum = 0;

        for i in 0..buffer.len(){
            requested_data.push(buffer[i] as char);
            if buffer[i] == 13 && buffer[i+1] == 10{
                sum+=buffer[i]+buffer[i+1];
                // data_start = i+3;
                // break;
            }
            
        }

        let mut data = String::new();

        for i in data_start..buffer.len(){
            data.push(buffer[i] as char);
        }

        println!("Data: {}",data);

        // for value in requested_data.split(" "){
        //     println!("{}",value);
        // }

        // println!("{}",count_blank);


        // for data in requested_data.lines(){
        //     println!("{}",data);
        // }
        // println!("{}",requested_data);

        // println!("{:?}",buffer);

        let _ = stream.write_all(response.as_bytes());

        // String::from_utf16_lossy(&buffer);

       
    }
    
    Ok(())
}