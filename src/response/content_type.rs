pub mod content_type{

    #[derive(Debug)]
    pub enum ContentyType{
        TextHtml,
        ApplicationJSON,
    }

    impl ContentyType {
        pub fn as_str(&self)->String{
            match self {
                Self::ApplicationJSON=>"application/json".to_string(),
                Self::TextHtml=>"text/html".to_string(),
            }
        }
    }
}