pub mod post_request{
    use std::collections::HashMap;
    use serde::Deserialize;
    use serde_json;
    use chrono::Local;
    
    use crate::request::request::request::RequestFormat;
    use crate::router::get::get_request::get_route_file_path;
    use crate::request::data_purified::data_purified::organized_data;
    use crate::request::request::request::file;
    use crate::errors::errors::errors::HttpErrors;
    use std::error::Error;
    use crate::errors::not_found_error404::not_found::NotFound;
    use crate::response::content_type::content_type::ContentyType;
   

    #[derive(Debug,Deserialize,Default)]    
    pub struct User{
        name:String,
        email:String,
        password:String,
    }


    
    pub fn post<'a>(requested_data:&RequestFormat,buffer:&[u8],data:String,bytes:usize)->std::result::Result<String,HttpErrors>{
       
        let url = get_route_file_path(&requested_data.request_line.url.url());

        let conetent_type = match &requested_data.header_lines.header.get("\nContent-Type"){
            Some(cnt_type)=>cnt_type.to_string(),
            None=>"application/json".to_string()
        };

        let data_starting_ind = match data.find("\r\n\r\n"){
            Some(i )=>i+4,
            None=>bytes,
        };  

        let org_map_data = match organized_data(&conetent_type, buffer.get(data_starting_ind..)){
            Some(data)=>data,
            None=>("".to_string(),HashMap::new())
        };

        if  url == "register.html".to_string(){
           let response = match register(&org_map_data.1){
            Ok(res)=>res,
            Err(e)=>{
                eprintln!("{}",e);
                
            }
           };
           println!("res: {}",response);
        } 
        
        Ok("".to_string())
    }

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