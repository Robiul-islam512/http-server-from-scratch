pub mod not_found{
    pub trait NotFoundSummarize {
        fn msg(&self)->String;
    }

    #[derive(Debug)]
    pub struct NotFound{
        status:String,
        content_type:String,
        date:String,
        content_length:usize,
        html_body:String,
    }


    impl NotFound {
        pub fn new(status:String,content_type:String,date:String,lenght:usize,html_body:String)->Self{
            Self { status, content_type, date, content_length: lenght, html_body }
        }
    }


    impl NotFoundSummarize for NotFound{
        fn msg(&self)->String {
            format!("{}\r\nContent-Type: {}\r\nContent-Length: {}\r\nDate: {}\r\n\r\n{}",self.status,self.content_type.as_str(),self.content_length,self.date,self.html_body)
        }
    }

    #[test]
    fn not_found_test(){
        
        use chrono::Local;
        use crate::response::content_type::content_type::ContentyType;

        let html_content = "<h1>Not Found</h1>";

        let not_found = NotFound{
            status:"HTTP/1.1 400 Bad Request".to_string(),
            content_type:ContentyType::ApplicationJSON.as_str(),
            content_length:html_content.as_bytes().len(),
            date:Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            html_body:html_content.to_string(),
        };

        let match_with =  format!("{}\r\nContent-Type: {}\r\nContent-Length: {}\r\nDate: {}\r\n\r\n{}",not_found.status,ContentyType::ApplicationJSON.as_str(),not_found.content_length,not_found.date,not_found.html_body);

        assert_eq!(not_found.msg(),match_with)

    }

}