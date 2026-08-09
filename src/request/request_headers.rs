pub mod RequestHeaders{
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct RequestHeaders<'a>{
        pub header:HashMap<&'a str,&'a str>
    }

    impl<'a> RequestHeaders<'a>{
        pub fn new(headers:&'a[String])->Self{
            let mut header:HashMap<&str,&str> = HashMap::new();

            for header_line in headers{
                let header_content:Vec<&str> = header_line.split(" ").collect();
                
                header.insert(header_content[0], header_content[1]);
            
            }

            RequestHeaders {  header }

        }
    }

}