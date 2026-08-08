pub mod URL{
    #[derive(Debug)]
    pub struct URL<'a>{
        url:&'a str,
    }

    impl<'a> URL<'a> {
        pub fn new(opt:Option<&'a str>)->Self{
            let url = match opt{
                Some(url)=>url,
                _=>{
                    return URL { url: "" };
                }
            };
            URL { url }
        }
    }

}

// fn url(option:Option<&str>)->URL{

//     let url_str = match option {
//         Some(url)=>url,
//         _=>{
//             return URL { url: "" };
//         }
//     };
    
//     return URL{
//         url:url_str,
//     }
// }
