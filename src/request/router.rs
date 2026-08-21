pub mod router{

    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use chrono::Local;
    
    use crate::response::response::response::{
        HtmlHeadersResponse, 
        HtmlResponse, 
        ResponseMessage, 
        StatusLine, 
        StatusMessage,
        Response,
        ResponseData,
        ResponseHeaderLines,
        Body
       
    };
    use crate::request::request_error::request_error::HttpError;
    use crate::request::data_decoding::data_decoding::data_extraction;

    pub trait GetRouteWiseResponse{
        fn get(&self)->std::result::Result<(String,String),Box<dyn std::error::Error>>;
    }

    pub trait PostRouteWiseResponse{
         fn post(&self)->std::result::Result<(String,String),Box<dyn std::error::Error>>;
    }

    #[derive(Debug)]    
    pub struct GetRouterComponents{
        method:String,
        url:String,
        file_path:String,
    }

    pub struct PostRouterComponents{
        url:String,
        content:String,
    }

    impl GetRouterComponents {
        pub fn new(method:String,url_path:String,file_path:String)->Self{
            GetRouterComponents { method, url:url_path,file_path }
        }
    }

    impl PostRouterComponents{
         pub fn new(url_path:String,content:String)->Self{
            PostRouterComponents { url:url_path, content }
        }
    }

    impl PostRouteWiseResponse for PostRouterComponents {
        fn post(&self)->std::result::Result<(String,String),Box<dyn std::error::Error>> {

            let status_line = StatusLine::new(
            String::from("HTTP/1.1"), 
            200, 
            StatusMessage::Ok,
            );



            let content = self.content.clone();

            let data = ResponseData{
                data:content,
            };

            let body = Body { 
                success: true, 
                message: "User Request Successfull".to_string(), 
                data
            };  

            let length = body.message().as_bytes().len();

            let header_lines = ResponseHeaderLines::new(
                String::from("Close"),
                Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                length,
                "application/json".to_string(),
            );

            let response = Response::new(
                status_line,
                header_lines,
                "\r\n\r\n",
                body
            );  

            println!("Big {:?}",response);

             match self.url.as_str() {
                "/register"=>{
                    Ok((response.message(),"".to_string()))
                },
                _=>{
                    return Err(
                        Box::new(HttpError::BadRequestError("post request route path doesn't matched".to_string()))
                    );
                }
            }
        }
    }


    impl GetRouteWiseResponse for GetRouterComponents {
        fn get(&self)->std::result::Result<(String,String),Box<dyn std::error::Error>>{
            
            let main_file = File::open(self.file_path.as_str());

            let html_content = match main_file {
              Ok(f)=>{
                    let read = BufReader::new(&f);

                    let mut html_content = String::new();

                    for content in read.lines(){
                        let content = content?;
                        html_content.push_str(&content);
                    }

                    html_content
              }
              Err(_)=>{
                "<h1>Server Error<h1>".to_string()
              }
            };

            let html_response_header = HtmlHeadersResponse::new(
            String::from("Close"),
            Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            html_content.as_bytes().len(),
            "text/html".to_string()
            );
            let status_line = StatusLine::new(
                String::from("HTTP/1.1"), 
                200, 
                StatusMessage::Ok,
            );
            

            let html_response = HtmlResponse::new(status_line, html_response_header);

            match self.url.as_str() {
                "/"=>{
                    Ok((html_response.message(),html_content))
                },
                "/register"=>{
                    Ok((html_response.message(),html_content))
                },
                "/login"=>{
                    Ok((html_response.message(),html_content))
                },
                "/todo"=>{
                    Ok((html_response.message(),html_content))
                },
                _=>{
                    return Err(
                        Box::new(HttpError::BadRequestError("route path doesn't matched".to_string()))
                    );
                }
            }
            
        }
    }

    pub fn post_router(url_path:String,content:String)->std::result::Result<(String,String),Box<dyn std::error::Error>>{
        let components = PostRouterComponents::new(
            url_path, 
            content
        );

        components.post()
    }

    pub fn get_router(method:&str,url_path:String,file_path:String)->std::result::Result<(String,String),Box<dyn std::error::Error>>{

        let components = GetRouterComponents::new(
            method.to_string(), 
            url_path.clone(),
            file_path,  
        );
        components.get()
    }
}