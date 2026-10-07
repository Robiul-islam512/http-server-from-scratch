pub mod parsing{

    use std::{collections::HashMap, str::from_utf8};

    use crate::parsing::{
        headers::headers::{extract_request_headers},
        request_line::request_line::{extract_request_line,RequestLine},
    };

    use crate::parsing::body::body::extract_body;

    #[derive(Debug)]
    pub struct Request{
        pub request_line:RequestLine,
        pub request_headers:HashMap<String,String>,
        pub body:Option<HashMap<String,String>>,
    }

    impl Request {
        pub fn new(request_line:RequestLine,request_headers:HashMap<String,String>,body:Option<HashMap<String,String>>)->Self{
            Self { request_line, request_headers, body }    
        }
    }

    pub fn parsing(buffer:[u8;4096],bytes_size:usize)->Request{
        
        let mut request_line = String::new();
        let mut request_end = 0;

        for (i,buf) in buffer.iter().enumerate(){
            let ch = *buf as char;
            if ch  == '\r'{
                request_end = i+2;
                break;
            } 
            request_line.push(ch);
        }   

        let line:Vec<String> = request_line.split_whitespace().map(|val|val.to_string()).collect();

        let mut headers = String::new();

        for i in request_end..bytes_size{
            let val = buffer[i] as char;

            if val != '{' {
                 headers.push(val);
            }
            else{
                request_end = i;
                break;
            }
        }

        let headers = String::from_utf8_lossy(&buffer).to_string();



        let request_headers_end = match headers.find("\r\n\r\n"){
            Some(u)=>u+4,
            None=>0
        };

        let request_line =  extract_request_line(line);
        let headers = extract_request_headers(headers);
    
        let is_post_request = request_line.method == "POST".to_string();

        if is_post_request{

            let body = extract_body(buffer,request_headers_end,&headers);

            return Request { request_line, request_headers: headers, body };
        }
        
        return Request { request_line, request_headers: headers, body: None };

    }


    pub fn handle_option(val:Option<&String>)->String{
        match val {
            Some(v)=>v.to_string(),
            None=>"".to_string()
        }
    }

}