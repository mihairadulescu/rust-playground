use crate::presentation::usage_of_types;

pub struct HttpConnection {
    pub url: &'static str,
    pub timeout: i32
}

pub trait HttpSend<T> {
    fn send(&self, payload:T);
}

pub fn usage() {
    let connection = HttpConnection{ url: "http://somepath.ro", timeout: 3000};
    connection.send(usage_of_types());
}