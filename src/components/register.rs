pub mod register {
    use std::collections::HashMap;
    use std::fs;
    use serde::{Deserialize,Serialize};

    use crate::errors::errors::errors::HttpErrors;

    use crate::components::login::login::{
        response_with_current_context,
        error_req,
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
            self.email
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

        let _400_ = missign_fields_status.0;

        let bad_req = error_req("bad req".to_string(), _400_);


        let server_error_msg = error_msg(
            "Server Error".to_string(),
            "Server facing error while fetching data from DB".to_string(),
        );

        let server_error = error_req("server error".to_string(),server_error_msg);
        
        let conflict_msg = error_msg(
            "User Conflict".to_string(),
            "User Alread Exists.Try new one".to_string(),
        );

        let user_conflict = error_req("user conflict".to_string(), conflict_msg);

        if missign_fields_status.1 == true {
            return Err(bad_req);
        }

        let mut users = match users_data("register.json") {
            Ok(users) => users,
            Err(_) => {
                return Err(server_error);
            }
        };
        let new_user = User {
            id: users.len() + 1,
            name,
            email,
            password,
        };


        let is_user_already_exists = alread_exists(&new_user, &users);

        if is_user_already_exists == true {
            return Err(user_conflict);
        }

        users.push(new_user.clone());

        let users_str = stringify(&users);

        let _ = fs::write("register.json", users_str);

        let content = new_user.get_user_info();

        let response = response_with_current_context(content, "Registration Successfull".to_string());

        
        Ok(response)
    }
}
