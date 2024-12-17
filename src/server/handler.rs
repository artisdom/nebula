use std::{
    fs,
    io::{Read, Write},
    net::TcpStream,
};

impl super::Server {
    pub fn handle_stream(&self, stream: &mut TcpStream) {
        let mut buf = [0; 1024];
        match stream.read(&mut buf) {
            Ok(bytes) => {
                if let Ok(request) = String::from_utf8(buf[..bytes].to_vec()) {
                    println!("Request received: \n{}", &request);
                    if let Some(first_line) = request.lines().next() {
                        let parts: Vec<&str> = first_line.split_whitespace().collect();
                        if parts.len() >= 2 {
                            let req_path = parts[1];
                            println!("Requested path: {}\n", req_path);

                            let mut full_path = self.container_path.clone();
                            full_path.push_str(req_path.trim_start_matches('/'));

                            println!("Full path: {}\n", &full_path);

                            let mut response =
                                String::from("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n");
                            if let Ok(paths) = fs::read_dir(full_path) {
                                response.push_str("<html><body>");
                                for path in paths {
                                    if let Ok(entry) = path {
                                        let file_name = entry.file_name();
                                        let file_name_str =
                                            file_name.to_str().unwrap();

                                        let mut href = String::from(req_path);
                                        if !href.ends_with('/') {
                                            href.push('/');
                                        }
                                        href.push_str(file_name_str);

                                        response.push_str("<a href='");
                                        response.push_str(&href);
                                        response.push_str("'>");
                                        response.push_str(file_name_str);
                                        response.push_str("</a><br>");
                                    }
                                }
                                response.push_str("</body></html>");
                            } else {
                                response = String::from(
                                    "HTTP/1.1 404 Not Found\r\n<h1>Path not found</h1>",
                                );
                            }
                            if let Err(e) = stream.write_all(response.as_bytes()) {
                                eprintln!("Failed to send response: {}", e);
                            }
                        } else {
                            eprintln!("Malformed request line: {}", first_line);
                        }
                    }
                }
            }
            Err(_) => {
                eprintln!("Can't handle the connection");
            }
        }
    }
}
