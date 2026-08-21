
pub mod response{
    use serde::Serialize;
    
    pub trait ResponseMessage {
        fn message(&self)->String;
    }


    #[derive(Debug)]
    pub enum StatusMessage{
        Ok,//200
        BadRequest,//400
    }   

    impl StatusMessage {
        pub fn msg(&self)->String{
            match self {
                Self::Ok=>"OK".to_string(),
                Self::BadRequest=>"Bad Request".to_string(),
            }
        }
    }   

    #[derive(Debug,Serialize)]
    pub struct ResponseData{
        pub data:String,
    }

    #[derive(Serialize,Debug)]
    pub struct Body{
        pub success:bool,
        pub message:String,
        pub data:ResponseData,
    }

    impl ResponseMessage for Body {
        fn message(&self)->String {
            let res_json =  serde_json::to_string(&self);
            
            let json_data = match res_json {
                Ok(data)=>data,
                Err(e)=>{
                    eprintln!("JSON ERROR: {}",e);
                    "".to_string()
                }
            };
            json_data
        }   
    }



    #[derive(Debug)]
    pub struct StatusLine{
        version:String,
        status_code:u32,
        message:StatusMessage,
    }

    impl StatusLine {
        pub fn new(version:String,status_code:u32,message:StatusMessage)->Self{
            StatusLine { version, status_code, message }
        }
    }


    #[derive(Debug)]
    pub struct ResponseHeaderLines{
        connection:String,
        date:String,
        content_length:usize,
        content_type:String,
    }

    impl ResponseHeaderLines {
        pub fn new(connection:String,date:String,content_length:usize,content_type:String)->Self{
            ResponseHeaderLines { connection, date, content_length, content_type }
        }
    }


    #[derive(Debug)]
    pub struct Response<'a>{
        status_line:StatusLine,
        header_lines:ResponseHeaderLines,
        blank_line:&'a str,
        body:Body,
    }


    impl<'a> Response<'a> {
        pub fn new(status_line:StatusLine,header_lines:ResponseHeaderLines,blank_line:&'a str,body:Body)->Self{
            Response { status_line, header_lines, blank_line, body }
        }
    }

    impl<'a> ResponseMessage for Response<'a> {
        fn message(&self)->String {
            let status_line = &self.status_line;
            let body = self.body.message();
            
            println!("Body: {}",body);

            return format!("{} {} {}\r\nConnection: {}\r\nDate: {}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",status_line.version,status_line.status_code,status_line.message.msg(),
              self.header_lines.connection,
              self.header_lines.date,
              self.header_lines.content_length,
              self.header_lines.content_type,
              body);
        }
    }

    pub struct HtmlHeadersResponse{
        connection:String,
        date:String,
        content_length:usize,
        content_type:String,
    }

    impl HtmlHeadersResponse {
        pub fn new(connection:String,date:String,content_length:usize,content_type:String)->Self{

            Self { connection, date, content_length, content_type }
            
        }
    }

    pub struct HtmlResponse{
        status_line:StatusLine,
        header_lines:HtmlHeadersResponse,
    }

    impl HtmlResponse {
        pub fn new(status_line:StatusLine,header_lines:HtmlHeadersResponse)->Self{
            Self { status_line, header_lines }
        }
    }

    impl ResponseMessage for HtmlResponse {
        fn message(&self)->String {
            format!("{} {} {}\r\n\
            Connection: {}\r\n\
            Date: {}\r\n\
            Content-Length: {}\r\n\
            Content-Type: {}\r\n\r\n",
            self.status_line.version,
            self.status_line.status_code,
            self.status_line.message.msg(),
              self.header_lines.connection,
              self.header_lines.date,
              self.header_lines.content_length,
              self.header_lines.content_type,
            )
        }
    }


    pub fn response(content_type:&str,content:String)->String{
    
        use crate::response::response::{json_content,html_content};

        let res =  match content_type {
            "application/json"=>{
                json_content(content)
            },
            "text/html"=>{
                html_content(content)
            },
            _=>{
                format!("server error")
            }
        };

        res
    }
    

}

pub fn html_content(content:String)->String{
    use chrono::Local;

    use crate::response::response::response::{
        StatusLine,
        StatusMessage,
        HtmlHeadersResponse,
        HtmlResponse,
        ResponseMessage
    };

    let html_response_header = HtmlHeadersResponse::new(
        String::from("Close"),
     Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
     content.as_bytes().len(),
      "text/html".to_string()
    );
    let status_line = StatusLine::new(
        String::from("HTTP/1.1"), 
        200, 
        StatusMessage::Ok,
    );
    
    let html_response = HtmlResponse::new(status_line, html_response_header);
    
    html_response.message()
    
}


pub fn json_content(content:String)->String{

    use chrono::Local;

    use crate::response::response::response::{
        StatusLine,
        StatusMessage,
        Response,
        ResponseData,
        ResponseHeaderLines,
        ResponseMessage,
        Body,
    };

    let status_line = StatusLine::new(
            String::from("HTTP/1.1"), 
            200, 
            StatusMessage::Ok,
            );



            let content = content;

            let data = ResponseData{
                data:content,
            };

            let body = Body { 
                success: true, 
                message: "User Request Successfull".to_string(), 
                data
            };  

            let length = body.message().as_bytes().len();

            let header_lines = ResponseHeaderLines::new(
                String::from("Close"),
                Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                length,
                "application/json".to_string(),
            );

            let response = Response::new(
                status_line,
                header_lines,
                "\r\n\r\n",
                body
            );

            response.message()
}