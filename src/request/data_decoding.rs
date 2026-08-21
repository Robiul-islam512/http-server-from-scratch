pub mod data_decoding{

    use serde::Serialize;
    use std::fs::write;

    pub trait ExtractedData {
        fn data(&self)->String;
    }


    #[derive(Debug,Serialize)]
    pub struct Registration{
        name:String,
        email:String,
        password:String,
    }   

    impl ExtractedData for Registration {
        fn data(&self)->String {
            format!("Name: {},Email: {}",self.name,self.email)
        }
    }
    pub struct LogIn{
        name_or_email:String,
        password:String,
    }


    pub fn data_extraction<'a>(request_data:&'a str)->String{

        
        let mut data = Vec::new();   
        let mut values = String::new();

        let symbols:Vec<(&str, &str)> = vec![("%40","@"),("%21","!"),("%20"," "),("%2B","+"),("%3D","="),("%26","&"),("%2F","/"),("%3F","?"),("%23","#"),("%5B","["),("%5D","]"),("%7B","{"),("%7D","}"),("%7C","|"),("%5E","^"),("%7E","~"),("%60","`"),("%3A",":"),("%3B",";"),("%27","'"),("%2C",","),("%3C","<"),("%3E",">"),("%25","%"),("%24","$"),("%28","("),("%29",")"),("%2A","*"),("%2D","-"),("%2E","."),("%5F","_")];

        let request_data = requested_data(0, symbols, request_data);
        
        for rd in request_data.chars(){
            if rd == '&'{
                data.push(values);
                values = String::new();
                continue;
            }
            
            values.push(rd);
        }

        data.push(values);

        let name = match data.get(0){
            Some(name)=>{
                let ind = match name.find("="){
                    Some(i)=>i,
                    None=>0,
                };
                name[ind+1..name.len()].to_string()

            },
            None=>"".to_string()
        };
        let email = match data.get(1){
            Some(email)=>{
               let ind = match email.find("="){
                    Some(i)=>i,
                    None=>0,
                };
                email[ind+1..email.len()].to_string()
            },
            None=>"".to_string()
        };

        let password = match data.get(2) {
            Some(pass)=>{
                let ind = match pass.find("="){
                    Some(i)=>i,
                    None=>0,
                };
                pass[ind+1..pass.len()-1].to_string()
            },
            None=>"".to_string()
        };

        let user_registration = Registration{
            name,
            email,
            password,
        };

        let users:Vec<String> = Vec::new();

        

        let user_json_data = match serde_json::to_string_pretty(&user_registration){
            Ok(val)=>{
                val
            },
            Err(_)=>{
                eprintln!("Somthing went while json parsing.");
                "".to_string()
            }
        };


        write("data.json", user_json_data);



        user_registration.data()

    }

    fn requested_data(i:usize,symbols:Vec<(&str,&str)>,data:&str)->String{
        
        if i >= symbols.len(){
            return data.to_string();
        }
        let data = data.replace(symbols[i].0, symbols[i].1);
        requested_data(i+1, symbols, &data)
    }

}