# Week5 mini rust project -- A HTTP fetcher
This project uses the TcpStream API in the standard library to make an HTTP request to the specified URL and retrieve the response. The response is then stored in a string and printed to the console.

Type `cargo run`

You will see something similar to

```
HTTP/1.1 200 OK
Date: Mon, 14 Feb 2022 10:30:50 GMT
Server: Apache
Last-Modified: Mon, 14 Feb 2022 08:30:50 GMT
Accept-Ranges: bytes
Content-Length: 4567
Connection: close
Content-Type: text/html
```
