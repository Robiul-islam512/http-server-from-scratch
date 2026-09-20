pub mod bad_request{
    use crate::response::content_type::content_type::ContentyType;

    use serde::Serialize;
    pub trait BadRequestFormat {
        fn msg(&self)->String;
    }

    #[derive(Debug,Serialize)]
    pub struct ErrorMessage{
        pub error:String,
        pub message:String,
    }

    pub struct BadReuqest<'a>{
        status:String,
        content_type:String,
        content_length:usize,
        error_msg:&'a str,
    }

    impl<'a> BadReuqest<'a> {
        pub fn new(status:String,content_type:String,length:usize,msg:&'a str)->Self{
            Self { status, content_type, content_length: length, error_msg: msg }
        }
    }


    impl<'a> BadRequestFormat for BadReuqest<'a> {
        fn msg(&self)->String {
            format!("{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",self.status,self.content_type,self.content_length,self.error_msg)
        }
    }

    #[test]
    fn test_bad_req(){
        let error_msg = ErrorMessage{
            error:"Bad Request".to_string(),
            message:"Request body could not be read properly.".to_string(),
        };

        let err_msg_str = match serde_json::to_string(&error_msg){
            Ok(msg)=>msg,
            Err(_)=>{
                "".to_string()
            }
        };

        println!("{}",err_msg_str);

        let bad_req = BadReuqest{
            status:"HTTP/1.1 400 Bad Request".to_string(),
            content_type:ContentyType::ApplicationJSON.as_str(),
            content_length:err_msg_str.as_bytes().len(),
            error_msg:&err_msg_str,
        };

        let demo_bad_req = format!("HTTP/1.1 400 Bad Request\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",ContentyType::ApplicationJSON.as_str(),&err_msg_str.as_bytes().len(),&err_msg_str);

        assert_eq!(demo_bad_req,bad_req.msg());

    }


}