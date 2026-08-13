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