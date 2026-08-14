pub mod response{
    use chrono::Local;
    use serde::Serialize;
    
    pub trait ResponseMessage {
        fn message(&self)->String;
    }


    #[derive(Debug)]
    enum StatusMessage{
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
        body:Body,
    }


    impl<'a> Response<'a> {
        fn new(status_line:StatusLine,header_lines:ResponseHeaderLines,blank_line:&'a str,body:Body)->Self{
            Response { status_line, header_lines, blank_line, body }
        }
    }

    impl<'a> ResponseMessage for Response<'a> {
        fn message(&self)->String {
            let status_line = &self.status_line;
            let body = self.body.message();
        

            return format!("{} {} {}\r\nConnection: {}\r\nDate: {}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",status_line.version,status_line.status_code,status_line.message.msg(),
              self.header_lines.connection,
              self.header_lines.date,
              self.header_lines.content_length,
              self.header_lines.content_type,
              body);
        }
    }

    pub fn response(content_type:&str,content:String)->String{

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
            data:data
        };  

        let length = body.message().as_bytes().len();

        let header_lines = ResponseHeaderLines::new(
            String::from("Close"),
            Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            length,
            content_type.to_string(),
        );

        let response = Response::new(
            status_line,
            header_lines,
            "\r\n\r\n",
            body
        );

        response.message()

    }
    

}