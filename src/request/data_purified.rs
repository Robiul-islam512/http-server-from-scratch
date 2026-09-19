pub mod data_purified{
    use std::collections::HashMap;
    use crate::request::data_decoding::data_decoding::data_decoding;

    pub fn organized_data<'a>(content_type:&'a str,u8_data:Option<&[u8]>)->Option<HashMap<String,String>>{
        let mut map_data:HashMap<String,String> = HashMap::new(); 
        let data = match u8_data {
            Some(d)=>{
                String::from_utf8_lossy(d).to_string()
            },
            None=>"".to_string()
        };  

        let data = data_decoding(&data);

        
        if content_type =="multipart/form-data" || content_type =="application/x-www-form-urlencoded" {
            let data_chars:Vec<char> = data.chars().collect();
            
            let mut split_vec = Vec::new(); 
            let mut d_str = String::new(); 
            for ch in data_chars{
                if ch != '\0'{
                    if ch == '&'{
                        split_vec.push(d_str);
                        d_str = String::new()
                    }
                    else{
                        d_str.push(ch);
                    }
                }
                else{
                    break;
                }
            }
            split_vec.push(d_str);  

            for kv in split_vec{
                let equal_sign_ind = match kv.find("="){
                    Some(i)=>i,
                    None=>0
                };

                let key = match kv.get(0..equal_sign_ind){
                    Some(k)=>k,
                    None=>""
                };
                let value = match kv.get(equal_sign_ind+1..){
                    Some(k)=>k,
                    None=>""
                };

                map_data.insert(key.to_string(), value.to_string());

            }
        }

        if content_type == "application/json"{

            let mut d = String::new();

            for ch in data.chars(){
                if ch == '\0'{
                    break;
                }
                d.push(ch);
                if ch == '{' || ch == ','{
                    d.push_str("\n\r");
                }

            }

            for line in d.lines(){  
                let line = line.trim();
                
                if line.contains(":"){
                    let col_ind = match line.find(":"){
                        Some(i)=>i,
                        None=>0
                    };

                    let key = get_val(line.get(..col_ind));
                    let val = get_val(line.get(col_ind+1..line.len()-1));
                    map_data.insert(key, val);
                }
            }

        }

        Some(map_data)

    }
    

    pub fn get_val(val:Option<&str>)->String{
        match val {
            Some(v)=>{
                    
                let mut values:Vec<char> = v.chars().collect();

                if values.first() == Some(&'\"'){
                     values.remove(0);
                }
                if values.last() == Some(&'\"'){
                    values.pop();
                }

                let v:String = values.iter().collect();
                v
            },
            None=>"".to_string(),
        }
    }

    #[test]
    fn test_organized_data_fn(){
        use super::data_purified::organized_data;
        
        let test_one = "name=khan&age=24".as_bytes();
        let test_tow = "name=frank&pass=4534".as_bytes();
        let test_three = "todo_name=reading_book&date=2020.12.05&time=12:55pm".as_bytes();
        let test_four = "name=robiul".as_bytes();

        let json_data_test_one = r#"{"name":"robiul","email":"robiux@gotmail.com","password":"123123"}"#.as_bytes();

        // println!("{}",json_data_test_one);

        let map_test_one = HashMap::from([
            ("name".to_string(),"khan".to_string()),
            ("age".to_string(),"24".to_string())
        ]);

        let map_test_tow = HashMap::from([
            ("name".to_string(),"frank".to_string()),
            ("pass".to_string(),"4534".to_string())
        ]);

        let map_test_three = HashMap::from([
            ("todo_name".to_string(),"reading_book".to_string()),
            ("date".to_string(),"2020.12.05".to_string()),
            ("time".to_string(),"12:55pm".to_string())
        ]);

        let map_test_four = HashMap::from([
            ("name".to_string(),"robiul".to_string())
        ]);


        let json_data_res_one = HashMap::from([
            ("name".to_string(),"robiul".to_string()),
            ("email".to_string(),"robiux@gotmail.com".to_string()),
            ("password".to_string(),"123123".to_string())
        ]);

        let res_five = match organized_data("application/json",Some(&json_data_test_one)){
            Some(data)=>data,
            None=>HashMap::new()
        };

        let res_one = match organized_data("application/x-www-form-urlencoded",Some(test_one)){
            Some(data)=>data,
            None=>HashMap::new()
        };
        let res_tow = match organized_data("application/x-www-form-urlencoded",Some(test_tow)){
            Some(data)=>data,
            None=>HashMap::new()
        };
         let res_three = match organized_data("application/x-www-form-urlencoded",Some(test_three)){
            Some(data)=>data,
            None=>HashMap::new()
        };

        let res_four = match organized_data("multipart/form-data",Some(test_four)) {
            Some(data)=>data,
            None=>HashMap::new()
        };
    

        assert_eq!(map_test_one,res_one);
        assert_eq!(map_test_tow,res_tow);
        assert_eq!(map_test_three,res_three);
        assert_eq!(map_test_four,res_four);
        assert_eq!(json_data_res_one,res_five);



    }

}