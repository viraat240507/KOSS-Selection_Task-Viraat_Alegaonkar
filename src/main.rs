use std::{sync::{Arc,Mutex},fs,io::{BufReader,prelude::*},net::{TcpStream,TcpListener}};
use koss_project::ThreadPool;
fn main()
{
    let listener=TcpListener::bind("127.0.0.1:8000").unwrap();
    let pool=ThreadPool::new(4);    
    let visitor_counter=Arc::new(Mutex::new(0));

    for stream in listener.incoming()
    {
        let stream=stream.unwrap();
        let counter=Arc::clone(&visitor_counter);

        pool.execute(move||{
            handle_connection(stream,counter);
        });
    }
}
fn handle_connection(mut stream:TcpStream,counter:Arc<Mutex<i32>>)
{
    let buf_reader=BufReader::new(&stream); //application of borrowing in Rust: &stream
    let http_request=buf_reader.lines().next().unwrap().unwrap();

    let (status_line,filename,content_type)=
    if http_request=="GET / HTTP/1.1"
    {
        let mut num=counter.lock().unwrap();
        *num+=1;
        ("HTTP/1.1 200 OK","HTML_file.html","text/html")
    }
    else if http_request=="GET /style.css HTTP/1.1"
    {
        ("HTTP/1.1 200 OK","style.css","text/css")
    }
    else if http_request=="GET /script.js HTTP/1.1"
    {
        ("HTTP/1.1 200 OK","script.js","application/javascript")
    }
    else 
    {
        ("HTTP/1.1 404 NOT FOUND","error.html","text/html")
    };

    if let Ok(contents)=fs::read_to_string(filename)
    {
        let length=contents.len();
        let response=format!("{status_line}\r\nContent-Type: {content_type}\r\nContent-length: {length}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
    }
    else 
    {
        let status_error="HTTP/1.1 404 NOT FOUND";
        let contents=fs::read_to_string("404.html").unwrap();
        let length=contents.len();
        let response=format!("{status_error}\r\nContent-Type: text/html\r\nContent-length: {length}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
    }
}
