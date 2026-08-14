pub mod Request{
    use std::collections::HashMap;

    use crate::request::request_error::RequestError::MessageFormate;
    use crate::response::content_type::ContentyType::ContentyType;
    use super::super::method::method::Method;
    use super::super::url::URL::URL;
    use super::super::request_headers::RequestHeaders::RequestHeaders;
    use super::super::request_line::RequestLine::RequestLine;
    use super::super::version::Version;

    use super::super::request_error::RequestError::HttpError;
    use super::super::request_error::RequestError::{ErrorBodyMessage,BadRequestError,BadRequestStatusLine,BadRequestMessage};
    

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

            content.insert("method".to_string(),format!("{:?}",self.request_line.method.method));
            content.insert("url".to_string(), format!("{:?}",self.request_line.url.url.trim()));
            content.insert("header".to_string(), format!("{:?}",self.header_lines.header));
            content.insert("body".to_string(), format!("{:?}",self.body.body.trim()));

            content
        }
    }
    
    pub fn request(buffer:[u8;4096],bytes:usize)->Result<HashMap<String,String>,HttpError>{
        let mut requested_data:Vec<String> = Vec::new();

        let mut data_str = String::new();

        for i in 0..bytes{
            if buffer[i] != 13 && buffer[i] !=10 {
                data_str.push(buffer[i] as char);
            }
            if buffer[i] == 13{
                requested_data.push(data_str);
                data_str = String::new();
            }
        } 

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

        let bad_req_status_line = BadRequestStatusLine{
            http_version:"HTTP/1.1".to_string(),
            status_code:400,
            request:"Bad Request".to_string(),
        };
       
        let error_msg = ErrorBodyMessage{
            error:"Bad Request".to_string(),
            message:"Request body could not be read properly.".to_string(),
        };  

        let body = match serde_json::to_string(&error_msg){
            Ok(val)=>val,
            Err(e)=>{
                return Err(
                    HttpError::FailedToSerialize(e.to_string())
                );
            }
        };

        let bad_req = BadRequestError{
            status_line:bad_req_status_line.message(),
            content_type:ContentyType::ApplicationJSON.as_str(),
            content_length:body.as_bytes().len(),
            body:error_msg,
        };
        
       

        let mut request_line =  request_lines.split(" ");
        
        let method_option = request_line.next();
        let url_option = request_line.next();
        let http_version_option = request_line.next();

        let method = Method::new(method_option);
        let url = URL::new(url_option);
        let version = Version::version(http_version_option);
        
        let request_line = RequestLine::new(&method, url, version);
        let header_lines = RequestHeaders::new(header_lines);


         if method.method.as_str() == "POST" && (request_lines.is_empty() || header_lines.header.is_empty() || data_str.len() == 0) {
            return Err(
                HttpError::BadRequestError(bad_req.msg(body))
            );
        }

        let mut entity_data = String::new();
        for body in data_str.split(" "){
            entity_data.push_str(body.trim());
        }
        
        let body = EntityBody{
            body:entity_data,
        };
        let requested_format = RequestFormat{
            request_line,
            header_lines,
            blank_line:"\r\n",
            body,
        };  

        Ok(requested_format.parse())

    }
}