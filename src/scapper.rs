/* extern crate select;
extern crate reqwest;
// use std::async_iter;

use select::document::Document;
use select::predicate::Name;
// use reqwest;


pub async fn scrap_link(url:&str) {
    let mut response = reqwest::get(url).await.unwrap();

    Document::from_read(response).unwrap().find(Name("a"))
    .filter_map(|n| n.attr("href"))
    .for_each(|x|  println!("{}",x));
    

} */