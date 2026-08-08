use std::net::{TcpListener,TcpStream};
use std::io::{Read, Result, Write};

mod method;
mod url;
mod version;

use method::Method::Method;
use url::URL::URL;
use version::Version;




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

    let mut buffer = [0;4096];
   
    for stream in litstener.incoming(){
        
        let mut stream = stream?;

        let _ = stream.read(&mut buffer);

        let mut requested_data:Vec<String> = Vec::new();

        let mut data_str = String::new();

        let mut ind = 0;

        for i in 0..buffer.len(){
            if buffer[i] != 13 && buffer[i] !=10{
                data_str.push(buffer[i] as char);
            }
            if buffer[i] == 13{
                requested_data.push(data_str);
                data_str = String::new();
            }    
        } 

        let mut request_line = requested_data[0].split(" ");
        let header_lines = &requested_data[1..requested_data.len()-2];
        let entity_body = &data_str;

        let method_option = request_line.next();
        let url_option = request_line.next();
        let http_version_option = request_line.next();


        let method = Method::new(method_option);
        let url = URL::new(url_option);
        let version = Version::version(http_version_option);
        
        let request_line = RequestLine{
            method,
            url,
            version,
        };

        println!("{:?}",request_line);
       
    }
    
    Ok(())
}


