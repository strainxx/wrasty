use std::{io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}, path::Path, time::{self, UNIX_EPOCH}};
use std::fs;
use std::collections::HashMap;

fn main() {
    println!("Starting server...");
    let listner = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Started!");
    for stream in listner.incoming(){
        let stream = stream.unwrap();
        println!("Handling connection!");
        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream){
    let buf = BufReader::new(&stream);
    let http_req: Vec<_> = buf.lines().map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();

    // println!("Request: {:#?}", http_req);
    let mut head = http_req.get(0).unwrap().split(' ');
    let _type = head.next().unwrap().to_string();
    let mut url = head.next().unwrap().to_string();
    url.remove(0);
    println!("Oh {url}");
    let mut ctx = HashMap::new();
    ctx.insert(String::from("time"), time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string());
    let resp = construct_responce(200, render_template(&url, ctx));
    stream.write_all(resp.as_bytes()).unwrap();
}

fn construct_responce(resp_code: u16, buff: String) -> String{
    let length = buff.len();
    let response_head = format!("HTTP/1.1 {resp_code} OK\r\nContent-Length: {length}\r\nServer: wrasty/beta\r\n\r\n{buff}");
    return response_head;
}

fn render_template(filename: &str, context: HashMap<String, String>) -> String{
    if !Path::new(filename).exists(){
        return fs::read_to_string("404.html").unwrap();
    }
    let mut template = fs::read_to_string(filename).unwrap();
    for (key, val) in context{
        template = template.replace(format!("[[ {key} ]]").as_str(), &val);
    }
    return template;
}