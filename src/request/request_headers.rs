pub mod request_header{
    use std::collections::HashMap;

   pub trait ParseHeader {
        fn parse(&self)->Vec<String>;
    }

    #[derive(Debug)]
    pub struct RequestHeaders<'a>{
        pub header:HashMap<&'a str,&'a str>
    }

    impl<'a> ParseHeader for RequestHeaders<'a> {
        fn parse(&self)->Vec<String> {
            
           let format = self.header
           .iter()
           .map(|header|format!("{}: {}",header.0,header.1))
           .collect();
            format
        }
    }

    impl<'a> RequestHeaders<'a>{
        pub fn new(headers:&'a[String])->Self{
            let mut header:HashMap<&str,&str> = HashMap::new();

            for header_line in headers{
                let clone_find = match header_line.find(":"){
                    Some(ind)=>ind,
                    None=>0,
                };

                
                let key = header_line.get(0..clone_find);
                let values = header_line.get(clone_find+1..);

                let key = match key {
                    Some(k)=>k,
                    None=>"",
                };

                let values = match values{
                    Some(v)=>v,
                    None=>"",
                };
                header.insert(key,values);
            }
            RequestHeaders {  header }

        }
    }

}