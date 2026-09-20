pub mod server_error{

    use chrono::Local;

    use super::super::bad_request400::bad_request::BadRequestFormat;

    pub struct ServerError{
        status:String,
        content_type:String,
        date:String,
        content_length:usize,
        error:String,
    }
    
    impl ServerError {
         pub fn new(status:String,content_type:String,date:String,lenght:usize,error_msg:String)->Self{
            Self { status, content_type, date, content_length: lenght, error:error_msg }
         }
    }

    impl<'a> BadRequestFormat for ServerError {
        fn msg(&self)->String {
            format!("{}\r\nContent-Type: {}\r\nContent-Length: {}\r\nDate: {}\r\n\r\n{}",self.status,self.content_type.as_str(),self.content_length,self.date,self.error)
        }
    }




}