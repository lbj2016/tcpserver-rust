use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    //绑定本地地址127.0.0.1，端口8888
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();

    //循环监听，接受请求
    for stream in listener.incoming() {
        let stream = stream.unwrap();//监听到一个TCP连接

        //处理该TCP请求
        handle_connection(stream);
    }
}

/**
 * 处理某个TCP请求
 */
fn handle_connection(mut stream: TcpStream) {
    //申明接收缓存区，大小为1024
    let mut buffer = [0; 1024];

    //读取数据至buffer
    stream.read(&mut buffer).unwrap();

    //使用from_utf8_lossy可以兼容无效的utf-8字符
    //The “lossy” part of the name indicates the behavior of this function when it sees 
    //an invalid UTF-8 sequence: it will replace the invalid sequence with �, the U+FFFD REPLACEMENT CHARACTER
    let req = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}", req);

    let request = req.to_string();
    //检查权限，只允许包含Alice的访问
    let response = match check_permission(&request) {
        Ok(0) => "Welcome, Alice \r\n",
        _ => "Sorry, not permited \r\n"
    };

    println!("sending to client: {}", response);
    //将响应写入缓冲区
    stream.write(response.as_bytes()).unwrap();
    //flush，发送给客户端
    stream.flush().unwrap();
}

fn check_permission(request: &str) -> Result<u8, &str>{
    if request.contains("Alice") {
        //只允许包含Alice的进行访问
        Ok(0)
    } else {
        Err("Not permited")
    }
}