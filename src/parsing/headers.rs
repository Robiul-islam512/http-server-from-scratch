pub mod headers{

    use std::collections::HashMap;

    pub fn extract_request_headers(request_headers:String)->HashMap<String,String>{
        
        let mut headers_map = HashMap::new(); 

        for header in request_headers.lines(){
            let line = header;
            
            let mut key = String::new();
            let mut val = String::new();
            let mut got_key = false;

            let mut colon_count = 0;

            for ch in line.chars(){
                
                if ch !=':' && !got_key{
                    key.push(ch);
                }
                else{   
                    colon_count+=1;
                    if ch == ':' && colon_count == 1{
                        got_key = true;
                        continue;
                    }
                    val.push(ch);
                }
            }
            let key = key.trim().to_string();
            let val = val.trim().to_string();

            if key.is_empty() || val.is_empty(){
                continue;
            }

            headers_map.insert(key, val);

        }

        headers_map

    }

}

#[cfg(test)]
mod tests{

    use std::collections::HashMap;

    use super::headers::extract_request_headers;

    #[test]
    fn headers_tests(){
        
        let testing_headers = "Host: localhost:8080\r\n\
User-Agent: curl/8.4.0\r\n\
Accept: */*\r\n\
Content-Type: application/json\r\n\
Content-Length: 27\r\n\
Authorization: Bearer abc123token\r\n\
Connection: keep-alive\r\n\
\r\n";

    let headers_map: HashMap<String, String> = HashMap::from([
        ("Host".to_string(), "localhost:8080".to_string()),
        ("User-Agent".to_string(), "curl/8.4.0".to_string()),
        ("Accept".to_string(), "*/*".to_string()),
        ("Content-Type".to_string(), "application/json".to_string()),
        ("Content-Length".to_string(), "27".to_string()),
        ("Authorization".to_string(), "Bearer abc123token".to_string()),
        ("Connection".to_string(), "keep-alive".to_string()),
    ]);
    
    let headers = extract_request_headers(testing_headers.to_string());


        assert_eq!(headers_map,headers);

    }
}