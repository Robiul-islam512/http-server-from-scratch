pub mod data_purified{
    use std::collections::HashMap;

use crate::response::response::response::StatusMessage::Ok;


    pub fn organized_data<'a>(u8_data:Option<&[u8]>)->Option<(String,HashMap<String,String>)>{
        let mut map_data:HashMap<String,String> = HashMap::new(); 
        let data = match u8_data {
            Some(d)=>{
                String::from_utf8_lossy(d).to_string()
            },
            None=>"".to_string()
        };  

        if data.contains("&"){
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

            println!("{:?}",map_data);
        }

        Some((data,map_data))

    }

    #[test]
    fn test_organized_data_fn(){
        use super::data_purified::organized_data;

        let test_one = "name=khan&age=24".as_bytes();
        let test_tow = "name=frank&pass=4534".as_bytes();
        let test_three = "todo_name=reading_book&date=2020.12.05&time=12:55pm".as_bytes();

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
            ("time".to_string(),"12:5pm".to_string())
        ]);

        let res_one = match organized_data(Some(test_one)){
            Some(data)=>data.1,
            None=>HashMap::new()
        };
        let res_tow = match organized_data(Some(test_tow)){
            Some(data)=>data.1,
            None=>HashMap::new()
        };
         let res_three = match organized_data(Some(test_three)){
            Some(data)=>data.1,
            None=>HashMap::new()
        };
        
        assert_eq!(map_test_one,res_one);
        assert_eq!(map_test_tow,res_tow);
        assert_eq!(map_test_three,res_three);



    }

}