pub mod parsing{
    use std::collections::HashMap;

    pub trait Parsing{
        
    }

    #[derive(Debug)]
    pub struct RequestLine{
        method:String,
        path:String,
        query_params:HashMap<String,String>,
        http_version:String,
    }

    impl RequestLine {
        fn new(method:String,path:String,query_params:HashMap<String, String>,http_version:String)->Self{
            Self { method, path, query_params, http_version }   
        }
    }

    pub fn parsing(buffer:[u8;4096],bytes_size:usize){
        
        let mut request_line = String::new();
        let mut request_line_end = 0;

        for (i,buf) in buffer.iter().enumerate(){
            let ch = *buf as char;
            if ch  == '\r'{
                request_line_end = i;
                break;
            } 
            request_line.push(ch);
        }   

        let line:Vec<&str> = request_line.split_whitespace().collect();

        let request_line =  extract_request_line(line);
        println!("{request_line:?}");
    }

    pub fn extract_request_line<'a>(request_line:Vec<&'a str>)->RequestLine{

        let method = handle_option(request_line.get(0));
        let route_path = handle_option(request_line.get(1));
        let http_version = handle_option(request_line.get(2));

        let route_nd_params = extract_route_nd_params(route_path);


        let route = route_nd_params.0;
        let params = route_nd_params.1;

        

       RequestLine::new(method.to_string(), route, params,http_version.to_string())
    }

    pub fn extract_route_nd_params<'a>(path:&'a str)->(String,HashMap<String,String>){
        let mut route = String::new();
        let mut params = String::new();

        let mut is_params_start = false;

        for ch in path.chars(){
            if ch!='?' && !is_params_start {
                route.push(ch);

            }
            else{
                is_params_start = true;
            }
            if is_params_start && ch!='?'{
                params.push(ch);
            }
        }

        (route, handle_params(params.clone()))
    }

    pub fn handle_params(params:String)->HashMap<String,String>{
        let params = params.replace("=", ":");
        let mut params_info = Vec::new();
        let mut params_map:HashMap<String,String> = HashMap::new();

        let mut val = String::new();
        for ch in params.chars(){
            if ch != '&'{
                val.push(ch);
            }
            else{
                params_info.push(val);
                val = String::new();
            }
        }

        params_info.push(val);

        for param in params_info.iter(){
            let mut key = String::new();
            let mut val = String::new();

            let mut got_key = false; 

            for v in param.chars(){
                if v != ':' && !got_key{
                    key.push(v);
                }
                else if v == ':'{
                    got_key = true;
                    continue;
                }
                else{
                    val.push(v);
                }
            }
            params_map.insert(key, val);

        }

        params_map

    }



    pub fn handle_option<'a>(val:Option<&&'a str>)->&'a str{
        match val {
            Some(v)=>v,
            None=>""
        }
    }

}