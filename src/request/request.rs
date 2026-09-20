pub mod request{
use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use crate::request::request_headers::request_header::ParseHeader;
    use crate::response::content_type::content_type::ContentyType;
    use super::super::method::method::Method;
    use super::super::url::url::URL;
    use super::super::request_headers::request_header::RequestHeaders;
    use super::super::request_line::request_line::RequestLine;
    use super::super::version::Version;

    use crate::errors::errors::errors::HttpErrors;
    use crate::errors::bad_request400::bad_request::{BadRequestFormat, BadReuqest, ErrorMessage};
    use crate::request::data_purified::data_purified::organized_data;


    pub trait RequestParse {
        fn parse(&self)->HashMap<String,String>;
    }

    pub trait BodyStringfy<'a> {
        fn body_as_str(&self)->String;
    }


    #[derive(Debug)]
    pub struct EntityBody{
        body:HashMap<String,String>
    }   

    impl<'a> BodyStringfy<'a> for EntityBody {
        fn body_as_str(&self)->String {
            let mut str_res = String::new();
            for (k,v) in &self.body{
                str_res.push_str(&format!("{}: {}",k,v).to_string());
            }              
            str_res
        }
    }


    #[derive(Debug)]
    pub struct RequestFormat<'a>{
        pub request_line:RequestLine<'a>,
        pub header_lines:RequestHeaders<'a>,
        pub blank_line:&'a str,
        pub body:String,
    }

    impl<'a> RequestParse for RequestFormat<'a> {
        fn parse(&self)->HashMap<String,String>{
            let mut content:HashMap<String,String> = HashMap::new();

            content.insert("method".to_string(),format!("{}",self.request_line.method.method.as_str()));
            content.insert("url".to_string(), format!("{}",self.request_line.url.url));
            content.insert("header".to_string(), format!("{:?}",self.header_lines.parse()));
            content.insert("body".to_string(), format!("{:?}",self.body));

            content
        }
    }
    
    pub fn request(buffer:[u8;4096],bytes:usize)->Result<HashMap<String,String>,HttpErrors>{

        let data = data_fetch(buffer, bytes).0;
        let requested_data = data_fetch(buffer, bytes).1;

        let data_starting_ind = body_starting_index(&data,bytes);


        let request_lines =  request_lines(&requested_data);

        let header_lines = header_lines(&requested_data);

         let error_msg = ErrorMessage{
            error:"Bad Request".to_string(),
            message:"Request body could not be read properly.".to_string(),
        };

        let err_msg_str = match serde_json::to_string(&error_msg){
            Ok(msg)=>msg,
            Err(_)=>{
                "".to_string()
            }
        };

        let bad_req = BadReuqest::new(
            "HTTP/1.1 400 Bad Request".to_string(),
            ContentyType::ApplicationJSON.as_str(),
            err_msg_str.as_bytes().len(),
            &err_msg_str
        );
       
        let mut request_line =  request_lines.split(" ");
        
        let method_option = request_line.next();
        let url_option = request_line.next();
        let http_version_option = request_line.next();

        let method = Method::new(method_option);
        let url = URL::new(url_option);
        let version = Version::version(http_version_option);
        
        let request_line = RequestLine::new(&method, url, version);
        let header_lines = RequestHeaders::new(header_lines);


        let requested_content_type = requeste_content_type(&header_lines);


        let actual_data = match organized_data( requested_content_type,buffer.get(data_starting_ind..)){
            Some(d)=>d,
            None=>HashMap::new()
        };
    
        if requested_content_type == "application/json" && actual_data.is_empty(){
            return Err(
                HttpErrors::BadRequest(bad_req.msg())
            );
        }

        if  request_lines.is_empty() || header_lines.header.is_empty() {
            return Err(
                HttpErrors::BadRequest(bad_req.msg())
            );
        }

        let body = EntityBody{
            body:actual_data,
        };

        let body_content = body.body_as_str();


        let requested_format = RequestFormat{
            request_line,
            header_lines,
            blank_line:"\r\n",
            body:body_content,
        };  
        
        // let x = post(&requested_format,buffer,bytes);

        Ok(requested_format.parse())

    }

    pub fn body_starting_index(data:&String,bytes:usize)->usize{
        match data.find("\r\n\r\n"){
            Some(i )=>i+4,
            None=>bytes,
        }
    }

    pub fn requeste_content_type<'a>(request_header:&'a RequestHeaders)->&'a str{
        match request_header.header.get("\nContent-Type"){
            Some(cnt_type)=>cnt_type,
            None=>"",
        }
    }

    pub fn header_lines<'a>(requested_data:&'a Vec<String>)->&'a [String]{
        let header_lines =  if requested_data.len()>2{
            requested_data.get(1..requested_data.len()-1)
        }else{
           None
        };

        match header_lines {
            Some(headers)=>headers,
            None=>&[]
        }
    }

    pub fn request_lines(requested_data:&Vec<String>)->&str{
        let request_lines =  requested_data.get(0);
        match request_lines {
            Some(line)=>line,
            None=>"",
        }
    }

    pub fn data_fetch(buffer:[u8;4096],bytes:usize)->(String,Vec<String>){
        let mut requested_data:Vec<String> = Vec::new();

        let mut data_str = String::new();

        let mut data = String::new();

        for i in 0..bytes{
            if i<bytes && buffer[i] != 13 && buffer[i+1] !=10 {
                data_str.push(buffer[i] as char);
            }
            if buffer[i] == 13{
                requested_data.push(data_str);
                data_str = String::new();
            }

            data.push(buffer[i] as char);
        } 
        (data,requested_data)
    }

    pub fn file(file_name:&str)->std::io::Result<String>{
    
    let f = File::open(file_name)?;
    let read = BufReader::new(&f);

    let mut html_content = String::new();

    for content in read.lines(){
        let content = content?;
        html_content.push_str(&content);
    }

    Ok(html_content)
}

}