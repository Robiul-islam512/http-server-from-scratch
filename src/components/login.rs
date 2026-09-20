pub mod login{
    use std::collections::HashMap;
    use chrono::Local;
    
    use crate::response::content_type::content_type::ContentyType;

    use crate::errors::{
        errors::errors::HttpErrors,
        not_found_error404::not_found::{NotFound,NotFoundSummarize},
    };

    use crate::router::post::post_request::{
        get_map_value,
        missing_fields,
    };


    pub fn login(data_map:&HashMap<String,String>)->std::result::Result<String,HttpErrors>{
        let name_or_email = get_email_or_name(data_map);
        let password = get_map_value(data_map, "password");

        let might_missing:Vec<(&str, &str)> = vec![(name_or_email.0,&name_or_email.1),("password",&password)]; 
        let missing_fields = missing_fields(might_missing);

        let msg = missing_fields.0;
        let not_found = NotFound::new(
            "HTTP/1.1 404 Not Found".to_string(),
            ContentyType::ApplicationJSON.as_str(),
            Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
           msg.as_bytes().len(),
            msg,
        );

        if missing_fields.1 == true{
            return Err(
                HttpErrors::NotFound(not_found.msg())
            );
        }

        // println!("{}",name_or_email);
        println!("{}",password);

        println!("{:?}",data_map);
        Ok("".to_string())
    }

    pub fn get_email_or_name<'a>(data_map:&HashMap<String,String>)->(&'a str,String){
        if data_map.contains_key("name"){
            ("name",get_map_value(data_map, "name"))
        }
        else{
            ("email",get_map_value(data_map, "email"))
        }
    }

}