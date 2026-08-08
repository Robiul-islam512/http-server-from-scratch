pub mod RequestLine{

    use crate::method::Method::Method;
    use crate::url::URL::URL;

    #[derive(Debug)]
    pub struct RequestLine<'a>{
        pub method:Method,
        pub url:URL<'a>,
        pub version:&'a str,
    }

    impl<'a> RequestLine<'a> {
        pub fn new(method:Method,url:URL<'a>,version: &'a str)->Self{
            RequestLine { method: method, url: url, version: version }
        }
    }

}