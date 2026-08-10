pub mod RequestHeaders{
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct RequestHeaders<'a>{
        pub header:HashMap<&'a str,Vec<&'a str>>
    }

    impl<'a> RequestHeaders<'a>{
        pub fn new(headers:&'a[String])->Self{
            let mut header:HashMap<&str,Vec<&'a str>> = HashMap::new();

            for header_line in headers{
                let header_content:Vec<&str> = header_line.split(" ").collect();
                
                let mut header_lines:Vec<&str> = Vec::new(); 

                for i in 1..header_content.len(){
                    header_lines.push(header_content[i]);
                    // println!("{}",header)
                }
                header.insert(header_content[0], header_lines);
            
            }

            RequestHeaders {  header }

        }
    }

}