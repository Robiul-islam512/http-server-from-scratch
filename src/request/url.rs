pub mod url{
    #[derive(Debug)]
    pub struct URL<'a>{
        pub url:&'a str,
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
