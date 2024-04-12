use std::{io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}, time::{self, UNIX_EPOCH}};
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

    println!("Request: {:#?}", http_req);
    let mut ctx = HashMap::new();
    ctx.insert(String::from("time"), time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string());
    let resp = construct_responce(200, render_template("index.html", ctx));
    stream.write_all(resp.as_bytes()).unwrap();
}

fn construct_responce(resp_code: u16, buff: String) -> String{
    let length = buff.len();
    let response_head = format!("HTTP/1.1 {resp_code} OK\r\nContent-Length: {length}\r\nServer: rustweb/beta\r\n\r\n{buff}");
    return response_head;
}

fn render_template(filename: &str, context: HashMap<String, String>) -> String{
    let mut template = fs::read_to_string(filename).expect("File must be created");
    for (key, val) in context{
        template = template.replace(format!("[[ {key} ]]").as_str(), &val);
    }
    return template;
}