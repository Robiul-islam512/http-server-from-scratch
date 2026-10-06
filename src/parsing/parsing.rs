pub mod parsing{

    use crate::parsing::{
        headers::headers::extract_request_headers,
        request_line::request_line::extract_request_line,
    };
    pub fn parsing(buffer:[u8;4096],bytes_size:usize){
        
        let mut request_line = String::new();
        let mut request_end = 0;

        for (i,buf) in buffer.iter().enumerate(){
            let ch = *buf as char;
            if ch  == '\r'{
                request_end = i+2;
                break;
            } 
            request_line.push(ch);
        }   

        let line:Vec<&str> = request_line.split_whitespace().collect();

        let mut headers = String::new();

        for i in request_end..bytes_size{
            let val = buffer[i] as char;

            if val != '{'{
                 headers.push(val);
            }
            else{
                request_end = i;
                break;
            }
        }

        let request_line =  extract_request_line(line);
        let headers = extract_request_headers(headers);
        
    }



    pub fn handle_option<'a>(val:Option<&&'a str>)->&'a str{
        match val {
            Some(v)=>v,
            None=>""
        }
    }

}