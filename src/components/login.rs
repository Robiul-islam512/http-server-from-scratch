pub mod login{
    use std::collections::HashMap;
    use chrono::Local;
use serde::Serialize;
    
    use crate::components::register::register::User;
    use crate::response::content_type::content_type::ContentyType;

    use crate::response::response::response::{
        Body,
        StatusLine,
        StatusMessage,
        ResponseHeaderLines,
        Response,
        ResponseMessage,
    };

    use crate::errors::{
        errors::errors::HttpErrors,
        not_found_error404::not_found::{NotFound,NotFoundSummarize},
        server_error::server_error::ServerError,
        bad_request400::bad_request::{BadReuqest,BadRequestFormat},
        conflict::conflict::ConflictError,
    };

    use crate::router::post::post_request::{
        get_map_value,
        users_data,
        error_msg,
    };

    pub struct Login{
        name_or_email:String,
        password:String,
    }

    #[derive(Default,Debug,Serialize)]
    pub struct UserInfo{
        pub id:usize,
        pub name:String,
        pub email:String,
    }




    pub fn login(data_map:&HashMap<String,String>)->std::result::Result<String,HttpErrors>{
        let name_or_email = get_email_or_name(data_map);
        let password = get_map_value(data_map, "password");

        println!("{:?}",data_map);

        let missing_count = [&name_or_email,&password].iter().filter(|f|f.is_empty()).count();

        let msg = if missing_count>1{
            "Login fields are missing.".to_string()
        }else{
            "Login field is missing".to_string()
        };

        let bad_req = error_req("bad req".to_string(), msg.clone());

        if name_or_email.is_empty() || password.is_empty(){
            return Err(
                bad_req
            );
        }

        let user_login_info = Login{
            name_or_email:name_or_email.clone(),
            password:password.clone()
        };  

          let server_error_msg = error_msg(
            "Server Error".to_string(),
            "Server facing error while fetching data from DB".to_string(),
        );

        let server_error = error_req("server error".to_string(), server_error_msg);

        let users = match users_data("register.json") {
            Ok(users) => users,
            Err(_) => {
                return Err(server_error);
            }
        };


        let login_status = login_logic(&user_login_info, &users);

        let response = serde_json::json!({
            "success":true,
            "message":"Logged in Successfull",
            "data":login_status.0
        });

        let body = match serde_json::to_string(&response){
            Ok(data)=>data,
            Err(_)=>"".to_string(),
        };


        if !login_status.1{
            let msg = "Login faild user not found.Please Register first!".to_string();
            let not_found = error_req("not found".to_string(), msg);
            println!("{}",not_found);
            return Err(not_found);
        }


        let response = response_with_current_context(body);

        Ok(response)

    }

    pub fn response_with_current_context(data_body:String)->String{


        let status_msg = StatusLine::new("HTTP/1.1".to_string(), 200, StatusMessage::Ok);

        let response_header_line = ResponseHeaderLines::new(
            "Close".to_string(),
            Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            data_body.as_bytes().len(),
            "application/json".to_string(),
        );

        let response = Response::new(status_msg, response_header_line, "", data_body);

        response.message()
    }

    pub fn error_req(error:String,msg:String)->HttpErrors{

        let bad_req_format = BadReuqest::new(
            "HTTP/1.1 400 Bad Request".to_string(), 
            "application/json".to_string(), 
            msg.as_bytes().len(), 
            &msg
        );

        let not_found_req_format = NotFound::new(
            "HTTP/1.1 404 Not Found".to_string(),
            ContentyType::ApplicationJSON.as_str(),
            Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
           msg.as_bytes().len(),
            msg.clone(),
        );

        let server_req_format = ServerError::new(
            "HTTP/1.1 500 Server Error".to_string(),
            ContentyType::ApplicationJSON.as_str(),
            Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            msg.clone().as_bytes().len(),
            msg.clone(),
        );
         let user_conflict_format = ConflictError::new(
            "HTTP/1.1 404 Server Error".to_string(),
            ContentyType::ApplicationJSON.as_str(),
            Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            msg.clone().as_bytes().len(),
            msg.clone(),
        );

        match error.as_str() {
            "bad req"=>HttpErrors::BadRequest(bad_req_format.msg()),
            "not found"=>HttpErrors::NotFound(not_found_req_format.msg()),
            "server error"=>HttpErrors::ServerError(server_req_format.msg()),
            "user conflict"=>HttpErrors::ConflictError(user_conflict_format.msg()),
            _=>HttpErrors::BadRequest("not matched".to_string()),
        }
    }

    pub fn login_logic(user_login_info:&Login,users:&Vec<User>)->(UserInfo,bool){
        let mut is_valid_user = false;

        // let mut matched_user_info;

        for user in users{
            if (user.email == user_login_info.name_or_email || user.name == user_login_info.name_or_email) && user.password == user_login_info.password{
                is_valid_user = true;
                if is_valid_user{
                    let  matched_user_info = UserInfo{
                        id:user.id,
                        name:user.name.clone(),
                        email:user.email.clone(),
                    };
                    return (matched_user_info,is_valid_user);
                }
            }
        }
        let empty = UserInfo::default();
        (empty,is_valid_user)
    }

    pub fn get_email_or_name<'a>(data_map:&HashMap<String,String>)->String{

        let name_or_email = vec![get_map_value(data_map, "name"),get_map_value(data_map, "email"),get_map_value(data_map,"name_or_email")];
        for val in name_or_email{
            if val.contains("@"){
                return val;
            }
            else if val.len()>1{
                return val;
            }
        }
        return "".to_string();
    }

    #[test]
    fn test_error_func(){
        let error_req = error_req("bad_req".to_string(),"Missing fiedls".to_string());

        println!("{:?}",error_req);

    }

}