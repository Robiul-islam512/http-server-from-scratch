
pub mod Method{

    #[derive(Debug)]
    enum MethodType{
        GET,
        POST,
        PUT,
        DELETE,
        HEAD, 
        NONE,
    }

    #[derive(Debug)]
    pub struct Method{
        method:MethodType
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




// fn method(option:Option<&str>)->Method{
//     return match option {
//             Some("GET")=>Method::GET,
//             Some("POST")=>Method::POST,
//             Some("PUT")=>Method::PUT,
//             Some("DELETE")=>Method::DELETE,
//             Some("HEAD")=>Method::HEAD,
//             _=>{
//                 println!("Unsupported Method");
//                 return Method::NONE;
//             },
//     };
// }