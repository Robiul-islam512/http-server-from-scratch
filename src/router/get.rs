pub mod get_request{

    use chrono::Local;
    
    use crate::response::response::response::{
        HtmlHeadersResponse, HtmlResponse, ResponseMessage,
    };
    use crate::response::response::response::{
        StatusLine,
        StatusMessage
    };
    
    
    pub fn get(content:&String)->String{
         let html_response_header = HtmlHeadersResponse::new(
            String::from("Close"),
            Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            content.as_bytes().len(),
            "text/html".to_string()
            );
            let status_line = StatusLine::new(
                String::from("HTTP/1.1"), 
                200, 
                StatusMessage::Ok,
            );

            let html_response = HtmlResponse::new(status_line, html_response_header,content.to_string());

            html_response.message()
    }
    

    pub fn get_route_file_path<'a>(route_path:&'a str)->String{
        let mut route = String::new();
                
        for ch in route_path.chars().rev(){
            if ch != '/'{
                route.push(ch);
            }
            else {
                break;
            }
        }

        if route_path.contains(".html"){
            let route:String = route.chars().rev().collect();
            return route;
        }

        if route.len() == 0{
            return "index.html".to_string();
        }
        let route:String = route.chars().rev().collect();
        format!("{}.html",route)
    }


    #[test]
    fn get_req_test(){
        use super::get_request::get_route_file_path;

        let route = "/register/user";
        let route_tow = "/todo/todo";
        let rout_three = "/";


        assert_eq!(get_route_file_path(route),"user.html".to_string());
        assert_eq!(get_route_file_path(route_tow),"todo.html".to_string());
        assert_eq!(get_route_file_path(rout_three),"home.html".to_string());
    }
}