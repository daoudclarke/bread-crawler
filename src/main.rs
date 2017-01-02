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

// extern crate curl;

// use std::io::{stdout, Write};

// use curl::easy::Easy;

// // Print a web page onto stdout
// fn main() {
//     let mut easy = Easy::new();
//     easy.url("http://www.tesco.com/groceries/product/search/
// default.aspx?searchBox=bread&icid=tescohp_sws-1_bread").unwrap();
//     easy.write_function(|data| {
//         Ok(stdout().write(data).unwrap())
//     }).unwrap();
//     easy.perform().unwrap();

//     println!("{}", easy.response_code().unwrap());
// }

// extern crate http;
// use http::client::RequestWriter;
// use http::method::Get;
// use http::status;
// use std::os;

// fn main() {
//     let request = RequestWriter::new(Get, FromStr::from_str(os::args()[1]).unwrap());
//     let response = match request.read_response() {
//         Ok(response) => response,
//         Err(_request) => unreachable!(), // Uncaught condition will have failed first
//     };
//     if response.status == status::Ok {
//         println!("Oh goodie, I got me a 200 OK response!");
//     } else {
//         println!("That URL ain't returning 200 OK, it returned {} instead", response.status);
//     }
// }



// extern crate hyper;
// use std::io::Read;
// use hyper::client::Request;

// // use hyper::request;

// fn main() {
//     println!("Hello, world!");

//     // let client = hyper::Client::new();
//     // let res = client.get("http://www.google.com").send().unwrap();

//     // let mut buffer = String::new();
//     // try!(res.read_to_string(&mut buffer));

//     let req = hyper::client::Request::get(hyper::Url::parse("www.google.com").unwrap()).unwrap();
//     let res = req.start()
//         .unwrap()
//         .send()
//         .unwrap()
//         .read_to_string()
//         .unwrap();

//     println!("Content: {}", res);
// }
