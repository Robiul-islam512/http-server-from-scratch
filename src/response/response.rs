pub mod Response{
    use chrono::Local;
    use super::super::content_type::ContentyType::{ContentyType};

    pub trait ResponseMessage {
        fn message(&self)->String;
    }

    #[derive(Debug)]
    enum StatusMessage{
        Ok,//200
        BadRequest,//400
    }

    #[derive(Debug)]
    struct ResponseEntityBody<'a>{
        body:&'a str,
    }

    impl<'a> ResponseEntityBody<'a> {
        fn new(body:&'a str)->Self{
            ResponseEntityBody { body }
        }
    }


    #[derive(Debug)]
    struct StatusLine{
        version:String,
        status_code:u32,
        message:StatusMessage,
    }

    impl StatusLine {
        fn new(version:String,status_code:u32,message:StatusMessage)->Self{
            StatusLine { version, status_code, message }
        }
    }


    #[derive(Debug)]
    struct ResponseHeaderLines{
        connection:String,
        date:String,
        content_length:usize,
        content_type:String,
    }

    impl ResponseHeaderLines {
        fn new(connection:String,date:String,content_length:usize,content_type:String)->Self{
            ResponseHeaderLines { connection, date, content_length, content_type }
        }
    }


    #[derive(Debug)]
    struct Response<'a>{
        status_line:StatusLine,
        header_lines:ResponseHeaderLines,
        blank_line:&'a str,
        body:ResponseEntityBody<'a>,
    }


    impl<'a> Response<'a> {
        fn new(status_line:StatusLine,header_lines:ResponseHeaderLines,blank_line:&'a str,body:ResponseEntityBody<'a>)->Self{
            Response { status_line, header_lines, blank_line, body }
        }
    }

    impl<'a> ResponseMessage for Response<'a> {
        fn message(&self)->String {
            return format!("{} {} {:?}\r\nconnection: {}\r\ndate: {}\r\ncontent-length: {}\r\ncontent-type: {}\r\n\r\n{}",self.status_line.version,self.status_line.status_code,self.status_line.message,self.header_lines.connection,self.header_lines.date,self.header_lines.content_length,self.header_lines.content_type,self.body.body);
        }
    }

    pub fn response(content_type:&str,content:String)->String{

        let content =  match content_type {
            "text/html"=>{
                content
            },
            "application/json"=>{
                "{}".to_string()
            },
            _=>{
                "".to_string()
            }
        };

        let status_line = StatusLine::new(
            String::from("HTTP/1.1"), 
            200, 
            StatusMessage::Ok,
        );

        let header_lines = ResponseHeaderLines::new(
            String::from("keep-alive"),
            Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            content.bytes().len(),
            content_type.to_string(),
        );

        let body = ResponseEntityBody::new(&content);

        let response = Response::new(
            status_line,
            header_lines,
            "",
            body
        );

        response.message()


    }
    

}