
pub mod response{
    use serde::Serialize;
    
    pub trait ResponseMessage {
        fn message(&self)->String;
    }


    #[derive(Debug)]
    pub enum StatusMessage{
        Ok
    }   

    impl StatusMessage {
        pub fn msg(&self)->String{
            match self {
                Self::Ok=>"Ok".to_string()
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
        pub data:String,
    }

    impl ResponseMessage for Body {
        fn message(&self)->String {
            let res_json =  serde_json::to_string(&self);
            
            match res_json {
                Ok(data)=>data,
                Err(_)=>"".to_string()
            }
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
        body:String,
    }


    impl<'a> Response<'a> {
        pub fn new(status_line:StatusLine,header_lines:ResponseHeaderLines,blank_line:&'a str,body:String)->Self{
            Response { status_line, header_lines, blank_line, body }
        }
    }

    impl<'a> ResponseMessage for Response<'a> {
        fn message(&self)->String {
            let status_line = &self.status_line;
            
            format!("{} {} {}\r\nConnection: {}\r\nDate: {}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",status_line.version,status_line.status_code,status_line.message.msg(),
              self.header_lines.connection,
              self.header_lines.date,
              self.header_lines.content_length,
              self.header_lines.content_type,
              self.body)
        }
    }

    #[derive(Debug)]
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
        body:String,
    }

    impl HtmlResponse {
        pub fn new(status_line:StatusLine,header_lines:HtmlHeadersResponse,body:String)->Self{
            Self { status_line, header_lines,body }
        }
    }

    impl ResponseMessage for HtmlResponse {
        fn message(&self)->String {
            let content_length = self.body.as_bytes().len();
            format!("{} {} {}\r\n\
            Connection: {}\r\n\
            Date: {}\r\n\
            Content-Length: {}\r\n\
            Content-Type: {}\r\n\r\n{}",
            self.status_line.version,
            self.status_line.status_code,
            self.status_line.message.msg(),
              self.header_lines.connection,
              self.header_lines.date,
              content_length,
              self.header_lines.content_type,
              self.body
            )
        }
    }

}

