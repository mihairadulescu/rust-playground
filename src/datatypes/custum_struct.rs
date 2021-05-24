use crate::network::{HttpSend, HttpConnection};

#[derive(Debug, Clone, Copy)]
pub struct CustomStruct {
    pub age:u32,
    pub title:&'static str
}

impl CustomStruct {
    pub fn new(age:u32, title:&'static str) -> CustomStruct {
        CustomStruct{age, title}
    }
}

impl HttpSend<CustomStruct> for HttpConnection {
    fn send(&self, payload: CustomStruct) {
        println!("Sending payload");
        println!("{}", payload.title);
    }
}