pub mod post_request{
    use std::collections::HashMap;
    use serde::Deserialize;
    use serde_json;
    use chrono::Local;
    
    use crate::request::request::request::RequestFormat;
    use crate::request::request_headers::request_header::RequestHeaders;
    use crate::router::get::get_request::get_route_file_path;
    use crate::request::data_purified::data_purified::organized_data;
    use crate::request::request::request::file;
    use crate::errors::errors::errors::HttpErrors;
    use std::error::Error;
    use crate::errors::not_found_error404::not_found::NotFound;
    use crate::response::content_type::content_type::ContentyType;
    use crate::request::request::request::{data_fetch,header_lines,requeste_content_type,body_starting_index};
   

    #[derive(Debug,Deserialize,Default)]    
    pub struct User{
        name:String,
        email:String,
        password:String,
    }


    
    pub fn post<'a>(data_map:&HashMap<String,String>,buffer:[u8;4096],bytes:usize)->std::result::Result<String,HttpErrors>{
        
        let url_path = url_path(data_map);
        
        let url = get_route_file_path(&url_path);

        let data = data_fetch(buffer, bytes);
        let header_lines = header_lines(&data.1);

        let header_lines = RequestHeaders::new(header_lines);
        let content_type = requeste_content_type(&header_lines);

        let body_starting_ind = body_starting_index(&data.0,bytes);

        let org_map_data = match organized_data(&content_type, buffer.get(body_starting_ind..)){
            Some(data)=>data,
            None=>HashMap::new()
        };

        if  url == "register.html".to_string(){
           let response = match register(&org_map_data){
            Ok(res)=>res,
            Err(e)=>{
                eprintln!("{}",e);
                "".to_lowercase()
            }
           };
        } 
        
        Ok("".to_string())
    }

    // pub fn url_path()

    pub fn register<'a>(body:&HashMap<String,String>)->std::result::Result<String,HttpErrors>{
        let name = get_map_value(body, "name");
        let email = get_map_value(body, "email");
        let password = get_map_value(body, "password");

        let fields:Vec<(&str,&str)> = vec![("name",&name),("email",&email),("password",&password)];

        let missgin_fields_status = missing_fields(fields);

        if missgin_fields_status.1 == true{
            return Err(
                HttpErrors::NotFound(missgin_fields_status.0)
            );
        }  

        Ok("".to_string())

    }

    pub fn missing_fields<'a>(fileds:Vec<(&str,&str)>)->(String,bool){
        let mut missing_fields = String::new();

        let mut is_missing = false;

        for field in fileds{
            if field.1.is_empty(){
                let f =  format!("{},",field.0);
                missing_fields.push_str(&f);
                is_missing = true;
            }
        }

         let _404_ = match file("404.html"){
            Ok(val )=>val,
            Err(_)=>"<h1>Not Found</h1>".to_string(),
        };


        let not_found = NotFound::new(
           "HTTP/1.1 400 Bad Request".to_string(),
            ContentyType::ApplicationJSON.as_str(), 
            Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), 
            _404_.as_bytes().len(),
            _404_
        );
       

        let msg = format!("You have sent missing or empty data of '{}'",missing_fields);
        (msg,is_missing)
    }

    fn url_path(data_map:&HashMap<String,String>)->String{
        match data_map.get("url"){
            Some(path)=>path.to_string(),
            None=>"/".to_string(),
        }
    }

    pub fn get_map_value<'a>(body:&HashMap<String,String>,key:&'a str)->String{
        match body.get(key) {
            Some(v)=>v.to_string(),
            None=>"".to_string(),
        }
    }

    #[test]
    fn post_req_test(){
        let testing_map = HashMap::from([
            ("name".to_string(),"gsdsd".to_string()),
            ("email".to_string(),"robiul&gamil.com".to_string()),
            ("password".to_string(),"".to_string()),
        ]);

        let x =  register(&testing_map);

        // let post = post(requested_data, buffer, data, bytes);

        println!("{:?}",x);

    }

}