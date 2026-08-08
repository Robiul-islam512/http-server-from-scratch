
pub mod Method{

    #[derive(Debug)]
    pub enum MethodType{
        GET,
        POST,
        PUT,
        DELETE,
        HEAD, 
        NONE,
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
