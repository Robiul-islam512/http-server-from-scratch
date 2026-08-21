pub mod router{

    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use chrono::Local;
    
    use crate::response::response::response::{HtmlHeadersResponse, HtmlResponse, ResponseMessage, StatusLine, StatusMessage};
    use crate::request::request_error::request_error::HttpError;

    pub trait RouteWiseResponse{
        fn get(&self)->std::result::Result<(String,String),Box<dyn std::error::Error>>;

        // fn post(&self)->std::result::Result<(String,String),Box<dyn std::error::Error>>;
        fn post(&self)->String;
    }

    #[derive(Debug)]    
    pub struct RouterComponents{
        method:String,
        url:String,
        file_path:String,
    }

    impl RouterComponents {
        pub fn new(method:String,url_path:String,file_path:String)->Self{
            RouterComponents { method, url:url_path,file_path }
        }
    }

    impl RouteWiseResponse for RouterComponents {
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
                        Box::new(HttpError::BadRequestError("route doesn't matched".to_string()))
                    );
                }
            }
            
        }

        // fn post(&self)->std::result::Result<(String,String),Box<dyn std::error::Error>> {}
        fn post(&self)->String {
            "".to_string()
        }

        // fn post(&self)->Result<(String,String),Box<dyn std::error::Error>> {}
    }

    pub fn router(method:&str,url_path:String,file_path:String)->std::result::Result<(String,String),Box<dyn std::error::Error>>{
        

        match method {
            "GET"=>{
                let components = RouterComponents::new(
                    method.to_string(), 
                    url_path.clone(),
                    file_path,  
                );

                components.get()
            }
            _=>{
                Ok(("".to_string(),"".to_string()))
            }
        }

        
        
    }
}