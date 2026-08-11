pub mod Request{
    use crate::request;

    use super::super::method::Method::Method;
    use super::super::url::URL::URL;
    use super::super::request_headers::RequestHeaders::RequestHeaders;
    use super::super::request_line::RequestLine::RequestLine;
    use super::super::version::Version;

    pub trait RequestParse {
        fn parse(&self)->Vec<String>;
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
        fn parse(&self)->Vec<String> {
            vec![
                format!("{:?}", self.request_line),
                format!("{:?}", self.header_lines.header),
                format!("{:?}", self.blank_line),
                format!("{:?}", self.body.body)
            ]
        }
    }

    
    pub fn request(buffer:[u8;4096],bytes:usize)->Vec<String>{
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

        let request_lines = Some(&requested_data[0]);
        let header_lines = requested_data.get(1..requested_data.len()-1);

        let request_lines = match request_lines {
            Some(line)=>line,
            None=>"",
        };

        let header_lines = match header_lines {
            Some(headers)=>headers,
            None=>&["".to_string()],
        };

        let mut request_line =  request_lines.split(" ");
        
        let method_option = request_line.next();
        let url_option = request_line.next();
        let http_version_option = request_line.next();

        let method = Method::new(method_option);
        let url = URL::new(url_option);
        let version = Version::version(http_version_option);
        
        let request_line = RequestLine::new(method, url, version);
        let header_lines = RequestHeaders::new(header_lines);

        let entity_body = data_str;


        let blank_line = "";
        let body = EntityBody{
            body:entity_body.clone()
        };
        let requested_format = RequestFormat{
            request_line,
            header_lines,
            blank_line,
            body,
        };  
        
println!("{:?}",requested_format);

        requested_format.parse()

    }
}