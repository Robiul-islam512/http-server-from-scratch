pub mod request{
    use std::borrow::Cow::{self, Borrowed};
use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use chrono::Local;

    use crate::request::request_error::request_error::MessageFormate;
    use crate::request::request_headers::request_header::ParseHeader;
    use crate::response::content_type::content_type::ContentyType;
    use super::super::method::method::Method;
    use super::super::url::url::URL;
    use super::super::request_headers::request_header::RequestHeaders;
    use super::super::request_line::request_line::RequestLine;
    use super::super::version::Version;

    // use super::super::request_error::request_error::HttpError;
    // use super::super::request_error::request_error::{ErrorBodyMessage,BadRequestError,BadRequestStatusLine,BadRequestMessage};
    use crate::errors::errors::errors::HttpErrors;
    use crate::errors::bad_request400::bad_request::{BadRequestFormat, BadReuqest, ErrorMessage};
    use crate::errors::not_found_error404::not_found::{NotFound};
    use crate::request::request::request::file as not_found_404;
    use crate::request::data_purified::data_purified::organized_data;


    pub trait RequestParse {
        fn parse(&self)->HashMap<String,String>;
    }

    #[derive(Debug)]
    struct EntityBody{
        body:String,
    }

    #[derive(Debug)]
    struct RequestFormat<'a>{
        request_line:RequestLine<'a>,
        header_lines:RequestHeaders<'a>,
        blank_line:&'a str,
        body:EntityBody,
    }

    impl<'a> RequestParse for RequestFormat<'a> {
        fn parse(&self)->HashMap<String,String>{
            let mut content:HashMap<String,String> = HashMap::new();

            content.insert("method".to_string(),format!("{}",self.request_line.method.method.as_str()));
            content.insert("url".to_string(), format!("{}",self.request_line.url.url));
            content.insert("header".to_string(), format!("{:?}",self.header_lines.parse()));
            content.insert("body".to_string(), format!("{:?}",self.body.body.trim()));

            content
        }
    }
    
    pub fn request(buffer:[u8;4096],bytes:usize)->Result<HashMap<String,String>,HttpErrors>{
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

        let data_starting_ind = match data.find("\r\n\r\n"){
            Some(i )=>i+4,
            None=>bytes,
        };

        let data = match organized_data( buffer.get(data_starting_ind..)){
            Some(d)=>d,
            None=>("".to_string(),HashMap::new())
        };

        
        println!("Data {:?}",data);


        let request_lines =  requested_data.get(0);

        let header_lines =  if requested_data.len()>2{
            requested_data.get(1..requested_data.len()-1)
        }else{
           None
        };

        let request_lines = match request_lines {
            Some(line)=>line,
            None=>"",
        };

        let header_lines = match header_lines {
            Some(headers)=>headers,
            None=>&["".to_string()],
        };

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


        

        let _404_ = match not_found_404("404.html"){
            Ok(val )=>val,
            Err(_)=>"<h1>Not Found</h1>".to_string(),
        };




        let not_found = NotFound::new(
           "HTTP/1.1 400 Bad Request".to_string(),
            ContentyType::ApplicationJSON.as_str(), 
            Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), 
            _404_.as_bytes().len(),
            _404_
        );


        // let bad_req_status_line = BadRequestStatusLine{
        //     http_version:"HTTP/1.1".to_string(),
        //     status_code:400,
        //     request:"Bad Request".to_string(),
        // };
       
        // let error_msg = ErrorBodyMessage{
        //     error:"Bad Request".to_string(),
        //     message:"Request body could not be read properly.".to_string(),
        // };  

        // let body = match serde_json::to_string(&error_msg){
        //     Ok(val)=>val,
        //     Err(e)=>"".to_string()
        // };

        // let bad_req = BadRequestError{
        //     status_line:bad_req_status_line.message(),
        //     content_type:ContentyType::ApplicationJSON.as_str(),
        //     content_length:body.as_bytes().len(),
        //     body:error_msg,
        // };
        
       

        let mut request_line =  request_lines.split(" ");
        
        let method_option = request_line.next();
        let url_option = request_line.next();
        let http_version_option = request_line.next();

        let method = Method::new(method_option);
        let url = URL::new(url_option);
        let version = Version::version(http_version_option);
        
        let request_line = RequestLine::new(&method, url, version);
        let header_lines = RequestHeaders::new(header_lines);


        let requested_content_type = match header_lines.header.get("Content-Type"){
            Some(cnt_type)=>{
                let find_semicolone = match cnt_type.find(";"){
                    Some(ind) =>ind,
                    None=>cnt_type.len(),
                };

                match cnt_type.get(0..find_semicolone) {
                    Some(val)=>val.to_string(),
                    None=>"".to_string(),
                }
            },
            None=>"".to_string()
        };

        if requested_content_type == "application/x-www-form-urlencoded".to_string(){
            // println!("{:?}",header_lines.header);
            // println!("{:?}",header_lines.header.get("body"));
        }

        // println!("{}",requested_content_type);



        

        if data_str.is_empty(){
            return Err(
                HttpErrors::BadRequest(bad_req.bad_request_format())
            );
        }

        if  request_lines.is_empty() || header_lines.header.is_empty() {
            return Err(
                HttpErrors::BadRequest(bad_req.bad_request_format())
            );
        }

        let body = EntityBody{
            body:data_str,
        };

        let requested_format = RequestFormat{
            request_line,
            header_lines,
            blank_line:"\r\n",
            body,
        };  

        

        Ok(requested_format.parse())

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