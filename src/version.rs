pub mod Version{
    pub fn version(opt:Option<&str>)->&str{
        match opt {
            Some(version)=>{
                version
            },
            _=>{
                ""
            }
        }
    }
}