pub mod Request{
    use super::super::method::Method::Method;
    use super::super::url::URL::URL;
    use super::super::request_headers::RequestHeaders::RequestHeaders;
    use super::super::request_line::RequestLine::RequestLine;
    use super::super::version::Version;

    #[derive(Debug)]
    struct EntityBody<'a>{
        body:&'a str,
    }

    #[derive(Debug)]
    struct RequestFormat<'a>{
        request_line:RequestLine<'a>,
        header_lines:RequestHeaders<'a>,
        blank_line:&'a str,
        body:EntityBody<'a>,
    }

    pub fn request(buffer:[u8;4096],bytes:usize)->(Method,usize){
        let mut requested_data:Vec<String> = Vec::new();

        let mut data_str = String::new();

        for i in 0..bytes{
            if buffer[i] != 13 && buffer[i] !=10{
                data_str.push(buffer[i] as char);
            }
            if buffer[i] == 13{
                requested_data.push(data_str);
                data_str = String::new();
            }
               
        } 

        let mut request_line = requested_data[0].split(" ");
        let header_lines = &requested_data[1..requested_data.len()-1];
        
        let method_option = request_line.next();
        let url_option = request_line.next();
        let http_version_option = request_line.next();

        let method = Method::new(method_option);
        let url = URL::new(url_option);
        let version = Version::version(http_version_option);
        
        let request_line = RequestLine::new(method, url, version);
        let header_lines = RequestHeaders::new(header_lines);

        let mut entity_body = String::new();

        let body_len = match header_lines.header.get("Content-Length") {
            Some(val)=> val[0].parse::<usize>().unwrap_or(0),
            None=>0,
        };

        let body_content_start = bytes-body_len; 

        for i in body_content_start..bytes{
            if buffer[i]!=13 && buffer[i]!=10 && buffer[i]!=34{
                entity_body.push(buffer[i] as char);
            }
        }

        let blank_line = "";
        let body = EntityBody{
            body:&entity_body
        };
        let requested_format = RequestFormat{
            request_line,
            header_lines,
            blank_line,
            body,
        };  

        println!("{:?}",requested_format);

        return (requested_format.request_line.method,body_len);

        

    }
}