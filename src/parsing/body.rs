pub mod body{
    use std::collections::HashMap;

    use crate::request::data_purified::data_purified::organized_data;
    use crate::parsing::parsing::parsing::handle_option;

    pub fn extract_body(buffer:[u8;4096],request_end:usize,headers:&HashMap<String,String>)->Option<HashMap<String,String>>{
        let content_length = handle_option(headers.get("Content-Length")).parse::<usize>().unwrap_or_else(|_|0);
            let content_type = handle_option(headers.get("Content-Type"));

            let request_body_bytes = buffer.get(request_end..request_end+content_length);

            let body = organized_data(&content_type, request_body_bytes);

            return body;
    }

}