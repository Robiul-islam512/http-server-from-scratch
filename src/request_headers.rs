pub mod RequestHeaders{
    #[derive(Debug)]
    pub struct RequestHeaders<'a>{
        pub header:Vec<&'a str>
    }

    impl<'a> RequestHeaders<'a>{
        pub fn new(headers:&'a[String])->Self{
            let mut req_headers:Vec<&str> = Vec::new();

            for header_line in headers{
                req_headers.push(header_line);
            }
            RequestHeaders { header: req_headers }

        }
    }

}