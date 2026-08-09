use std::net::{TcpListener,TcpStream};
use std::io::{Read, Result, Write};
use std::str::Bytes;
use chrono::{DateTime, Local};

mod request;

#[derive(Debug)]
enum StatusMessage{
    Ok,//200
    BadRequest,//400
}

// #[derive(Debug)]
// enum ContentType{
//     ApplicationJson,
//     TextHtml,
//     TextPlain,
// }

#[derive(Debug)]
struct ResponseEntityBody<'a>{
    body:&'a str,
}

#[derive(Debug)]
struct StatusLine{
    version:String,
    status_code:u32,
    message:StatusMessage,
}

#[derive(Debug)]
struct ResponseHeaderLines{
    connection:String,
    date:String,
    content_length:usize,
    content_type:String,
}

#[derive(Debug)]
struct Response<'a>{
    status_line:StatusLine,
    header_lines:ResponseHeaderLines,
    blank_line:&'a str,
    body:ResponseEntityBody<'a>,
}



fn main()->Result<()>{
    let litstener = TcpListener::bind("127.0.0.1:8080")?;

    
//    let todays_date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    let status_line = StatusLine{
        version:String::from("HTTP/1.1"),
        status_code:200,
        message:StatusMessage::Ok,
    };

    let content = "my name is robiul";
    let mut content_len = 0;

    for ch in content.chars(){
        content_len+=1;
    }

    let header_lines = ResponseHeaderLines{
        connection:String::from("Close"),
        date:Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        content_length:content_len,
        content_type:String::from("application/json"),
    };

    let body = ResponseEntityBody{
        body:content,
    };

    let response = Response{
        status_line,
        header_lines,
        blank_line:"",
        body,
    };

    let response = format!("{} {} {:?}\r\nconnection: {}\r\ndate: {}\r\ncontent-length: {}\r\ncontent-type: {}\r\n\r\n{}",response.status_line.version,response.status_line.status_code,response.status_line.message,response.header_lines.connection,response.header_lines.date,response.header_lines.content_length,response.header_lines.content_type,response.body.body);

    
    for stream in litstener.incoming(){
        let mut buffer = [0;4096];
        let mut stream = stream?;

        let bytes =  stream.read(&mut buffer)?;

        let info =  request::request::Request::request(buffer, bytes);

        stream.write_all(response.as_bytes())?;

       
    }
    
    Ok(())
}


