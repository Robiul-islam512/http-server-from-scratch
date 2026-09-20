pub mod errors{
    use std::fmt::{Display,Result};
    
    #[derive(Debug)]
    pub enum HttpErrors {
        NotFound(String),
        BadRequest(String),
        ServerError(String),
        ConflictError(String),

    }

    impl Display for HttpErrors {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
            match self {
                Self::NotFound(msg)=>write!(f,"{}",msg),
                Self::BadRequest(msg)=>write!(f,"{}",msg),
                Self::ServerError(msg)=>write!(f,"{}",msg),
                Self::ConflictError(msg)=>write!(f,"{}",msg),
            }
        }
    }   

    
    impl std::error::Error for HttpErrors {}

    // #[test]
    // fn test_errors(){
    //     use chrono::Local;
    //     use crate::errors::not_found_error404::not_found::NotFound;
    //     use crate::response::content_type::content_type::ContentyType;

    //     let content = "</h1>Not found </h2>";

    //     let not_found_error = NotFound{
    //         status:"HTTP/1.1 404 Not Found".to_string(),
    //         content_type:ContentyType::TextHtml,
    //         date:Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    //         content_length:content.as_bytes().len(),
    //         html_body:content.to_string(),
    //     };

    //     let expected_error = not_found_error.msg();
    //     let err = HttpErrors::NotFound(expected_error.clone());

    //     println!("{}",expected_error);
    //     println!("{}",err);
    //     assert_eq!(err.to_string(),expected_error);


    // }


}