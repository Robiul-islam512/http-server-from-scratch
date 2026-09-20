pub mod register {
    use std::collections::HashMap;
    use std::fs;
    use chrono::Local;
    use serde::{Deserialize,Serialize};

    use crate::response::content_type::content_type::ContentyType;
    use crate::errors::errors::errors::HttpErrors;
    use crate::errors::{
        not_found_error404::not_found::{NotFound,NotFoundSummarize},
        server_error::server_error::{ServerError},
        conflict::conflict::ConflictError,
        bad_request400::bad_request::BadRequestFormat,
    };
    
    use crate::response::response::response::{
        Body,
        StatusLine,
        StatusMessage,
        ResponseHeaderLines,
        Response,
        ResponseMessage,
    };

    use crate::router::post::post_request::{
        get_map_value,
        missing_fields,
        error_msg,
        users_data,
        alread_exists,
        stringify,
    };
    
    #[derive(Debug,Deserialize,Default,Serialize,Clone)]    
    pub struct User{
        pub id:usize,
        pub name:String,
        pub email:String,
        pub password:String,
    }
    impl User {
        pub fn get_user_info(&self)->String{
           format!(
            r#"{{"name":"{}","email":"{}"}}"#,
            self.name,
            self.password
            )
        }
    }
    pub fn register<'a>(body: &HashMap<String, String>) -> std::result::Result<String, HttpErrors> {
        let name = get_map_value(body, "name");
        let email = get_map_value(body, "email");
        let password = get_map_value(body, "password");

        let fields: Vec<(&str, &str)> =
            vec![("name", &name), ("email", &email), ("password", &password)];

        let missign_fields_status = missing_fields(fields);

        let _404_ = missign_fields_status.0;

        let not_found = NotFound::new(
            "HTTP/1.1 404 Not Found".to_string(),
            ContentyType::ApplicationJSON.as_str(),
            Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            _404_.as_bytes().len(),
            _404_,
        );

        let server_error_msg = error_msg(
            "Server Error".to_string(),
            "Server facing error while fetching data from DB".to_string(),
        );

        let server_error = ServerError::new(
            "HTTP/1.1 500 Server Error".to_string(),
            ContentyType::ApplicationJSON.as_str(),
            Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            server_error_msg.as_bytes().len(),
            server_error_msg,
        );

        let conflict_msg = error_msg(
            "User Conflict".to_string(),
            "User Alread Exists.Try new one".to_string(),
        );

        let user_conflict = ConflictError::new(
            "HTTP/1.1 404 Server Error".to_string(),
            ContentyType::ApplicationJSON.as_str(),
            Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            conflict_msg.as_bytes().len(),
            conflict_msg,
        );

        if missign_fields_status.1 == true {
            return Err(HttpErrors::NotFound(not_found.msg()));
        }

        let mut users = match users_data("register.json") {
            Ok(users) => users,
            Err(_) => {
                return Err(HttpErrors::ServerError(server_error.msg()));
            }
        };
        let new_user = User {
            id: users.len() + 1,
            name,
            email,
            password,
        };

        println!("{:?}", users);

        let is_user_already_exists = alread_exists(&new_user, &users);

        if is_user_already_exists == true {
            return Err(HttpErrors::ConflictError(user_conflict.msg()));
        }

        users.push(new_user.clone());

        let users_str = stringify(&users);

        let _ = fs::write("register.json", users_str);

        let content = match serde_json::to_string(&new_user) {
            Ok(d) => d,
            Err(_) => "".to_string(),
        };

        let body = Body {
            success: true,
            message: "Registration Successfull".to_string(),
            data: new_user.get_user_info(),
        };

        let status_msg = StatusLine::new("HTTP/1.1".to_string(), 200, StatusMessage::Ok);

        let response_header_line = ResponseHeaderLines::new(
            "Close".to_string(),
            Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            body.message().as_bytes().len(),
            "application/json".to_string(),
        );

        let response = Response::new(status_msg, response_header_line, "", body.message());

        // println!("{}",response.message());

        // println!("{:?}",users);

        Ok(response.message())
    }
}
