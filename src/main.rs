use rustweb::context::{Context, ContextFn};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let web = rustweb::build_server("127.0.0.1", 7878);
    web.get("/hello", hello_handler);
    web.post("/update", update_handler);
    web.post("/file/upload", upload_handler);
    web.post("/file/multipart", multipart_handler);
    web.run();
}

fn hello_handler(mut c: Context) {
    let content = "{\"code\":200,\"message\":\"\"}";
    c.json(content.as_bytes());
}

fn update_handler(mut c: Context) {
    let content = "{\"code\":200,\"message\":\"\"}";
    // let body = c.request.body();
    // println!("\n{}", String::from_utf8(body).unwrap());
    // println!("body: len = {}",body.len());
    c.request.parse_post_form();
    println!("form: {:?}", c.request.post_form);
    c.json(content.as_bytes());
}

fn upload_handler(mut c: Context) {
    let mut buf = vec![0; 8192];
    let mut file = File::create("test.img").unwrap();

    let mut total = 0usize;
    let length: usize = c.request.header_first("Content-Length").parse().unwrap();

    loop {
        match c.request.read_body(&mut buf) {
            Ok(0) => {
                println!("End");
                break;
            }
            Ok(n) => {
                file.write_all(&buf[0..n]).unwrap();
                total += n;
                println!("length = {length}, total = {total}, n = {n}");
                if total >= length {
                    println!("last: {:?}", &buf[n - 300..n]);
                    println!("Finish");
                    break;
                }
            }
            Err(e) => {
                println!("upload_handler error: {}", e);
                break;
            }
        }
    }
    file.flush().unwrap();
    let metadata = file.metadata().unwrap();
    println!("Recv file size: {}", metadata.len());
    let content = "{\"code\":200,\"message\":\"Upload finish!\"}";
    c.json(content.as_bytes());
}

fn multipart_handler(mut c: Context) {
    let reader = c.request.multipart();
    // reader.next(&mut c.request.reader);

    // Part 1
    let part = reader.next().unwrap();
    println!("disposition1: {}", part.disposition);
    println!("contentType1: {:?}", part.content_type);

    let body = reader.part_body().unwrap();
    println!("body1: {}", String::from_utf8(body.to_vec()).unwrap());

    // Part 2
    let part = reader.next().unwrap();
    println!("disposition2: {}", part.disposition);
    println!("contentType2: {:?}", part.content_type);

    let body = reader.part_body().unwrap();
    println!("body2: {}", String::from_utf8(body.to_vec()).unwrap());

    // Part 3
    let part = reader.next().unwrap();
    println!("disposition3: {}", part.disposition);
    println!("contentType3: {:?}", part.content_type);

    let body = reader.part_body().unwrap();
    println!("body3: {}", String::from_utf8(body.to_vec()).unwrap());
    println!("body3: {:?}", body);

    // Part 4
    let part = reader.next().unwrap();
    println!("disposition4: {}", part.disposition);
    println!("contentType4: {:?}", part.content_type);

    let body = reader.part_body().unwrap();
    println!("body4: {}", String::from_utf8(body.to_vec()).unwrap());

    c.ok();
}
