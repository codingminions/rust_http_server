# rust_http_server
**Description :** An implementation of a http server from scratch in Rust.
**References:** [Official Rust Programming Language](https://doc.rust-lang.org/book/ch00-00-introduction.html)
## Overview: 
I am following the official documentation of Rust Programming language to learn and practice Rust. The purpose of this whole exercise is to
develop in-depth understanding of Rust programming and its usage in creating systems like Vector Dbs, Client-side ML frameworks among many. 
However, I did not want to read the whole documentation at first and then move to their last section which is building a HTTP server from scratch. So, I have started to implement the project in increments while learning the theory as and when i come across those concepts in development process. I am maintaining a [Reading list](./http/reading_list.md) which highlights the concepts I learnt on each day of the development sprint. 

## Getting Started
`cd http`\
`cargo build`\
`cargo run`

### Output:
```
Compiling http v0.1.0 (/Users/prateekkumar/zym/rust_grpc_server/http)
Finished dev [unoptimized + debuginfo] target(s) in 0.69s
Running 'target/debug/http'
Public path is: /Users/prateekkumar/zym/rust_http_server/http/public
Server is listening on 127.0.0.1:8080
```

Next you should open up a new tab in any browser client that supports HTTP/1.1 and make the api calls:
1. **Method: "GET", Path: "/", "Query_string":"", URL: 127.0.0.1:8080**

   Output: 
   ```
    Stream: TcpStream { addr: 127.0.0.1:8080, peer: 127.0.0.1:61164, fd: 4 } OK
    received a request: GET / HTTP/1.1
    Host: 127.0.0.1:8080
    Connection: keep-alive
    sec-ch-ua: "Chromium";v="123", "Not:A-Brand";v="8"
    sec-ch-ua-mobile: ?0
    sec-ch-ua-platform: "macOS"
    DNT: 1
    Upgrade-Insecure-Requests: 1
    User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36
    Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7
    Sec-Fetch-Site: none
    Sec-Fetch-Mode: navigate
    Sec-Fetch-User: ?1
    Sec-Fetch-Dest: document
    Accept-Encoding: gzip, deflate, br, zstd
    Accept-Language: en-US,en;q=0.9
    Cookie: _grpcui_csrf_token=l4eNuRFyuAbDe-ZLFCkdQ662wWNZ8zWd6fI33ROYZfI
    Stream: TcpStream { addr: 127.0.0.1:8080, peer: 127.0.0.1:61165, fd: 4 } OK
    received a request: GET /style.css HTTP/1.1
    Host: 127.0.0.1:8080
    Connection: keep-alive
    sec-ch-ua: "Chromium";v="123", "Not:A-Brand";v="8"
    DNT: 1
    sec-ch-ua-mobile: ?0
    User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36
    sec-ch-ua-platform: "macOS"
    Accept: text/css,*/*;q=0.1
    Sec-Fetch-Site: same-origin
    Sec-Fetch-Mode: no-cors
    Sec-Fetch-Dest: style
    Referer: http://127.0.0.1:8080/
    Accept-Encoding: gzip, deflate, br, zstd
    Accept-Language: en-US,en;q=0.9
    Cookie: _grpcui_csrf_token=l4eNuRFyuAbDe-ZLFCkdQ662wWNZ8zWd6fI33ROYZfI
    ```

2. **Method: "GET", Path:"/hello", Query_string: "", URL: 127.0.0.1:8080**

   Output:
   ```
    Stream: TcpStream { addr: 127.0.0.1:8080, peer: 127.0.0.1:61167, fd: 4 } OK
    received a request: GET /hello HTTP/1.1
    Host: 127.0.0.1:8080
    Connection: keep-alive
    sec-ch-ua: "Chromium";v="123", "Not:A-Brand";v="8"
    sec-ch-ua-mobile: ?0
    sec-ch-ua-platform: "macOS"
    DNT: 1
    Upgrade-Insecure-Requests: 1
    User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36
    Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7
    Sec-Fetch-Site: none
    Sec-Fetch-Mode: navigate
    Sec-Fetch-User: ?1
    Sec-Fetch-Dest: document
    Accept-Encoding: gzip, deflate, br, zstd
    Accept-Language: en-US,en;q=0.9
    Cookie: _grpcui_csrf_token=l4eNuRFyuAbDe-ZLFCkdQ662wWNZ8zWd6fI33ROYZfI
    

3. **Method: "GET", Path:"/", Query_string:"a=2&b=4&c&d&d=3&d=6", URL=127.0.0.1:8080**

   Output:
    ```
    GET /?/test?a=2&b=4&c&d&d=3&d=6 HTTP/1.1
    Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7
    Accept-Encoding: gzip, deflate, br, zstd
    Accept-Language: en-US,en;q=0.9
    Cache-Control: max-age=0
    Connection: keep-alive
    Cookie: _grpcui_csrf_token=l4eNuRFyuAbDe-ZLFCkdQ662wWNZ8zWd6fI33ROYZfI
    DNT: 1
    Host: 127.0.0.1:8080
    Sec-Fetch-Dest: document
    Sec-Fetch-Mode: navigate
    Sec-Fetch-Site: none
    Sec-Fetch-User: ?1
    Upgrade-Insecure-Requests: 1
    User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36
    sec-ch-ua: "Chromium";v="123", "Not:A-Brand";v="8"
    sec-ch-ua-mobile: ?0
    sec-ch-ua-platform: "macOS"
    ``` 

### Provisions covered for successful parse:
**Methods**: [[GET, DELETE, POST, PUT, HEAD, CONNECT, OPTIONS, TRACE,PATCH]](https://github.com/codingminions/rust_http_server/blob/main/http/src/httpmods/method.rs#L19)

**ParseError**: [InvalidRequest, InvalidEncoding, InvalidProtocol, InvalidMethod], implemented the Debug & Display trait for [struct ParseError](https://github.com/codingminions/rust_http_server/blob/main/http/src/httpmods/request.rs#L68).

**StatusCode**: [[Ok = 200, BadRequest = 400, NotFound = 404]](https://github.com/codingminions/rust_http_server/blob/main/http/src/httpmods/status_code.rs#L4)

**Request**: [[path: &'buf str, query_string: Option<QueryString<'buf>>, method: Method]](https://github.com/codingminions/rust_http_server/blob/main/http/src/httpmods/request.rs#L10) and we read the stream using *read* macros and make sure we are covering the scenarios of <u>dangling pointers/references</u> with the help of **lifetimes**.

**Response**: [[status_code: StatusCode, body: Option<String>]](https://github.com/codingminions/rust_http_server/blob/main/http/src/httpmods/response.rs#L5) and we write this response back to the stream using *write* macros.