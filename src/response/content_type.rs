pub mod content_type{

    #[derive(Debug)]
    pub enum ContentyType{
        TextHtml,
        ApplicationJSON,
        ApplicationXWwwFormUrlendcoded,
        MultipartFormData,
        // multipart/form-data;
        // "application/x-www-form-urlencoded"

    }

    impl ContentyType {
        pub fn as_str(&self)->String{
            match self {
                Self::ApplicationJSON=>"application/json".to_string(),
                Self::TextHtml=>"text/html".to_string(),
                Self::ApplicationXWwwFormUrlendcoded=>"application/x-www-form-urlencoded".to_string(),
                Self::MultipartFormData=>"multipart/form-data".to_string(),
            }
        }
    }
}