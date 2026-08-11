use std::net::{TcpListener};
use std::io::{BufRead, BufReader, Read, Result, Write};
use serde_json::Value;
use std::fs::{File, read, write};

mod request;
mod response;

fn main()->Result<()>{
    let litstener = TcpListener::bind("127.0.0.1:8080")?;
    
    for stream in litstener.incoming(){
        let mut buffer = [0;4096];
        let mut stream = stream?;

        let bytes =  stream.read(&mut buffer)?;

        let data_info =  request::request::Request::request(buffer, bytes);

        let response = response::response::Response::response();

        // for data in data_info.iter(){
        //     for (k,v) in data{
        //         println!("{} {}",k,v);
        //     }
        //     println!();
        //     // println!("{} {}",k,v);
        // }

        let user_body_data = match data_info.get("body"){
            Some(body)=>body,
            None=>"",
        };

        let f = File::open("data.json")?;

        let mut read =  BufReader::new(f);

        for line in read.lines(){
            let line = line?;
            write("data.json", &user_body_data)?;
            println!("{}",line);
        }

        // print!("{:?}",read);


        // println!("{}",json_val);
        // println!("{}",user_body_data);

        // println!("{:?}",data_info);
        stream.write_all(response.as_bytes())?;

        // println!("Request Info: {:?}", info);

        
       
    }
    
    Ok(())
}


