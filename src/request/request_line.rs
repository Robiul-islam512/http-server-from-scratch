pub mod RequestLine{

    use super::super::method::method::Method;
    use super::super::url::URL::URL;

    #[derive(Debug)]
    pub struct RequestLine<'a>{
        pub method:&'a Method,
        pub url:URL<'a>,
        pub version:&'a str,
    }

    impl<'a> RequestLine<'a> {
        pub fn new(method:&'a Method,url:URL<'a>,version: &'a str)->Self{
            RequestLine { method: method, url: url, version: version }
        }
    }

}