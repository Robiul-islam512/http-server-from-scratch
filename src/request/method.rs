
pub mod method{

    #[derive(Debug)]
    pub enum MethodType{
        GET,
        POST,
        PUT,
        DELETE,
        HEAD, 
        NONE,
    }

    impl MethodType {
        pub fn as_str(&self)->String{
            match self {
                Self::GET=>"GET".to_string(),
                Self::POST=>"POST".to_string(),
                Self::DELETE=>"DELETE".to_string(),
                Self::HEAD=>"HEAD".to_string(),
                Self::PUT=>"PUT".to_string(),
                Self::NONE=>"NONE".to_string(),
            }
        }
    }

    #[derive(Debug)]
    pub struct Method{
        pub method:MethodType
    }

    impl Method{
        pub fn new(opt:Option<&str>)->Self{
            let method =  match opt {
                Some("GET")=>MethodType::GET,
                Some("POST")=>MethodType::POST,
                Some("PUT")=>MethodType::PUT,
                Some("DELETE")=>MethodType::DELETE,
                Some("HEAD")=>MethodType::HEAD,
                _=>{
                    MethodType::NONE
                },
            };
            Method { method }
        }
    }
}
