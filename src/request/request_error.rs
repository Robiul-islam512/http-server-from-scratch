pub mod RequestError{
    use serde::Serialize;
    use std::fmt;
    use std::error;

    pub trait BadRequestMessage {
        fn message(&self)->String;
    }

    #[derive(Serialize,Debug)]
    pub struct ErrorBodyMessage{
        pub error:String,
        pub message:String,
    }

    pub struct BadRequestStatusLine{
        pub http_version:String,
        pub status_code:u32,
        pub request:String,
    }   
 
    impl BadRequestMessage for BadRequestStatusLine {
        fn message(&self)->String {
            format!("{} {} {}",self.http_version,self.status_code,self.request)
        }
    } 
    #[derive(Debug)]
     pub struct BadRequestError{
        pub status_line:String,
        pub content_type:String,
        pub content_length:usize,
        pub body:ErrorBodyMessage,
    } 
    
    #[derive(Debug)]
    pub enum HttpError{
        BadRequestError(String),
        FailedToSerialize(String)
    }

    impl fmt::Display for HttpError{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                HttpError::BadRequestError(msg)=>write!(f,"{}",msg),
                HttpError::FailedToSerialize(msg)=>write!(f, "Failed to serialize: {}",msg),
            }
        }
    }

    impl error::Error for HttpError {}

    pub trait MessageFormate {
        fn msg(&self,body:String)->String;
    }

    impl MessageFormate for BadRequestError {
        fn msg(&self,body:String)->String {

            let status_line = &self.status_line;
            let content_type = &self.content_type;
            let body = body;
            let content_lenght = body.as_bytes().len();
            
            let response_msg = format!(
                "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",status_line,content_type,content_lenght,body
            );
            response_msg
        }
    }

}