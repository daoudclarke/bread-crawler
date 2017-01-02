extern crate reqwest;

use std::io::Read;

fn main() {
    let mut response = reqwest::get("http://www.tesco.com/groceries/product/search/\
                                     default.aspx?searchBox=bread&icid=tescohp_sws-1_bread")
        .expect("Failed to send request");
    println!("{}", response.status());
    for header in response.headers().iter() {
        println!("{}: {}", header.name(), header.value_string());
    }
    let mut buf = String::new();
    response.read_to_string(&mut buf).expect("Failed to read response");
    println!("{}", buf);
}

