pub mod conflict{
    use super::super::bad_request400::bad_request::BadRequestFormat;

    pub struct ConflictError{
        status:String,
        content_type:String,
        date:String,
        content_length:usize,
        error:String,
    }
    
    impl ConflictError {
        pub fn new(status:String,content_type:String,date:String,lenght:usize,error_msg:String)->Self{
            Self { status, content_type, date, content_length: lenght, error:error_msg }
         }
    }

    impl  BadRequestFormat for ConflictError {
        fn msg(&self)->String {
            format!("{}\r\nContent-Type: {}\r\nContent-Length: {}\r\nDate: {}\r\n\r\n{}",self.status,self.content_type.as_str(),self.content_length,self.date,self.error)
        }
    }


}