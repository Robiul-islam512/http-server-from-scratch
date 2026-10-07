pub mod  request_line{
    use std::collections::HashMap;
    use crate::parsing::parsing::parsing::handle_option;

    #[derive(Debug,PartialEq)]
    pub struct RequestLine{
        pub method:String,
        pub path:String,
        pub query_params:HashMap<String,String>,
        pub http_version:String,
    }

    impl RequestLine {
        pub fn new(method:String,path:String,query_params:HashMap<String, String>,http_version:String)->Self{
            Self { method, path, query_params, http_version }   
        }
    }

     pub fn extract_request_line(request_line:Vec<String>)->RequestLine{

        let method = handle_option(request_line.get(0));
        let route_path = handle_option(request_line.get(1));
        let http_version = handle_option(request_line.get(2));

        println!("{:?}",request_line);

        let route_nd_params = extract_route_nd_params(&route_path);


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

        println!("{}",params);

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


}

#[cfg(test)]
mod tests{
    use std::collections::HashMap;
    use super::request_line::{extract_request_line,RequestLine};

    #[test]
    fn tests(){
        let line1 = vec!["GET".to_string(), "/users/42?sort=asc&page=2".to_string(), "HTTP/1.1".to_string()];
        let line2= vec!["POST".to_string(),"/users/login.html".to_string(),"HTTP/1.1".to_string()];

        let request_line_test_with_params = extract_request_line(line1);

        let sec_request_line_without_params = extract_request_line(line2);

        // println!("{:?}",request_line_test_with_params);

        let test_params = HashMap::from([
            ("page".to_string(),"2".to_string()),
            ("sort".to_string(),"asc".to_string())
        ]);

        let request_line_with_params = RequestLine::new("GET".to_string(),"/users/42".to_string() , test_params, "HTTP/1.1".to_string());

        let mut sec_query_params = HashMap::new();
        sec_query_params.insert("".to_string(), "".to_string());

        let sec_request_line = RequestLine::new("POST".to_string(), "/users/login.html".to_string(), sec_query_params,"HTTP/1.1".to_string());
        


        assert_eq!(request_line_test_with_params,request_line_with_params);
        assert_eq!(sec_request_line_without_params,sec_request_line);
    }

}