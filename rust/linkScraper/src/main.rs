extern crate curl;
extern crate scraper;

use curl::easy::Easy;
use std::env::args;
use scraper::{Html, Selector};

fn get(a: &str, vec: &mut Vec<u8>){
    vec.clear();
    let mut easy = Easy::new();
    easy.url(a).unwrap();

    let mut transfer = easy.transfer();
    transfer.write_function(|data| {
        vec.extend_from_slice(data);
        Ok(data.len())
    }).unwrap();
    transfer.perform().unwrap();
}

fn parse_links(vec: Vec<u8>, links: &mut Vec<String>){
    let buffer  = String::from_utf8(vec).unwrap();
    let fragment = Html::parse_document(buffer.as_str());
    let anchor = Selector::parse("a").unwrap();

    for element in fragment.select(&anchor) {
        let x =  element.value().attr("href");
        if x == None{
            continue;
        }

        let mut s = x.unwrap().replace("\"","");
        s = s.replace("'", "");
        links.push(s);
    }
}

fn main(){
    let args: Vec<_> = args().collect();
    let mut page: Vec<u8> = Vec::new();
    let mut links: Vec<String> = Vec::new();

    get(args[1].as_str(), &mut page);
    parse_links(page, &mut links);

    for x in links{
        println!("Found [{}]", x);
    }
}
