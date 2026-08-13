pub mod ContentyType{
    // use serde_json::
    // trait ResponseType {
    //     fn response_type(&self)->Self;
    // }

    // #[derive(Debug)]
    // pub struct TextHtml{
    //     text_html:String
    // }
    
    //  #[derive(Debug)]
    // pub struct TextPlain{
    //     text_plain:String,
    // }

    //  #[derive(Debug)]
    // pub struct ApplicationJSON{
    //     application_json:String,
    // }

    //  #[derive(Debug)]
    // pub struct ImagePng{
    //     img_png:String,
    // }

    //  #[derive(Debug)]
    // pub struct ImageJpeg{
    //     image_jpeg:String,
    // }

    // pub enum ImageType{
    //     ImagePng,
    //     ImageJpeg
    // }

    // #[derive(Debug)]
    // pub struct MultipartFormData{
    //     form_data:String,
    // }

    #[derive(Debug)]
    pub enum ContentyType{
        TextHtml,
        ApplicationJSON,
    }

    impl ContentyType {
        pub fn as_str(&self)->String{
            match self {
                Self::ApplicationJSON=>"application/json".to_string(),
                Self::TextHtml=>"text/html".to_string(),
            }
        }
    }


    // impl TextHtml{
    //     fn new(&self)->Self{
    //         TextHtml { text_html: (sel }
    //     }
    // }

    // impl ResponseType for TextHtml {
    //     fn response_type(&self)->String {
    //         TextHtml { text_html: s }
    //     }
    // }

}