pub mod post_request{
    use std::collections::HashMap;
    use std::fs;
    

    use crate::request::request_headers::request_header::RequestHeaders;
    use crate::router::get::get_request::get_route_file_path;
    use crate::request::data_purified::data_purified::organized_data;
    use crate::errors::errors::errors::HttpErrors;
    use crate::request::request::request::{data_fetch,header_lines,requeste_content_type,body_starting_index};
    use crate::request::request_error::request_error::ErrorBodyMessage;
    use crate::components::{
        register::register::{register,User},
        login::login::login,
    };




    
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

        if  url.to_lowercase() == "register.html".to_string(){
            return register(&org_map_data);
           
        } 
        else if url.to_lowercase() == "login.html".to_string(){
            return login(&org_map_data);
        }
        
        Ok("".to_string())
    }

    pub fn stringify(users:&Vec<User>)->String{
         match serde_json::to_string_pretty(users){
            Ok(cnt)=>cnt,
            Err(_)=>"".to_string(),
        }
    }

    pub fn alread_exists(new_user:&User,prev_users:&Vec<User>)->bool{
        
        for user in prev_users{
            if new_user.name.to_lowercase() == user.name.to_lowercase() || new_user.email.to_lowercase() == user.email.to_lowercase(){
                return true;
            }
        }
        false
    }

    pub fn users_data(file_name:&str)->std::result::Result<Vec<User>,Box<dyn std::error::Error>>{
        
        let read_to_string = fs::read_to_string(file_name)?;

        let users:Vec<User> = serde_json::from_str(&read_to_string)?;

        Ok(users)

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

        let msg = format!("You have sent missing or empty data of '{}'",missing_fields);
        (msg,is_missing)
    }

    pub fn error_msg(error:String,msg:String)->String{
        let msg = ErrorBodyMessage{
            error,
            message:msg
        };

        let str_msg = match serde_json::to_string(&msg){
            Ok(msg)=>msg,
            Err(_)=>"".to_string()
        };
        str_msg
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

        println!("{:?}",x);

    }

}