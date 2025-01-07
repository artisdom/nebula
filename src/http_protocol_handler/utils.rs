use std::{fs, net::TcpStream, path::Path, string::FromUtf8Error};
use std::io::{BufRead, BufReader};
use chrono::Utc;

pub fn decompress(stream: &mut TcpStream) -> Result<String, FromUtf8Error> {
    let mut reader = BufReader::new(stream);
    let received: Vec<u8> = reader. fill_buf().unwrap().to_vec();
    reader.consume(received.len());
    let result = String::from_utf8(received)?; // TODO: Handle non utf8 cases;
    Ok(result)
}

pub fn read_file_content_as_string(path: &String) -> Result<String, String> {
    if !Path::new(&path).exists() {
        return Err(format!("File does not exist: {}", path));
    }
    let content = fs::read_to_string(path).unwrap();
    
    Ok(content)
}


pub fn mount(file_type: &String, file_content: &String) -> String {
    let current_date = Utc::now().format("%a, %d %b %Y %T GMT").to_string();

    let mut ftype = String::new();
    if file_type == ".js" {
        ftype = String::from("application/javascript");
    } else if file_type == ".html" {
        ftype = String::from("text/html");
    } else if file_type == ".css" {
        ftype = String::from("text/css");
    }

    let result = format!(
        "HTTP/1.1 200 OK\n\
        Date: {}\n\
        Content-Type: {}; charset=UTF-8\n\
        Content-Length: {}\n\
        Connection: close\n\
        \n\
        {}",
        current_date,
        ftype,
        file_content.len(),
        file_content
    );

    result
}

pub fn mount_not_founded() -> String {
    let content = r#"
    <html>
    <head>
        <style>
            body {
                font-family: Arial, sans-serif;
                display: flex;
                justify-content: center;
                align-items: center;
                height: 100vh;
                margin: 0;
                background-color: #f0f0f0;
                color: #333;
            }
            .container {
                text-align: center;
                background-color: #fff;
                padding: 30px;
                border-radius: 8px;
                box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
                width: 80%;
                max-width: 600px;
            }
            h1 {
                font-size: 72px;
                color: #e74c3c;
                margin-bottom: 20px;
            }
            h4 {
                font-size: 24px;
                color: #555;
            }
            .footer {
                font-size: 14px;
                color: #999;
                margin-top: 30px;
            }
        </style>
    </head>
    <body>
        <div class="container">
            <h1>404</h1>
            <h4>Page Not Found</h4>
            <div class="footer">Oops! The page you're looking for doesn't exist.</div>
        </div>
    </body>
    </html>
    "#;

    let result = format!("\
                    HTTP/1.1 404 Not Found\n\
                    Content-Type: text/html; charset=UTF-8\n\
                    Content-Length: {}\n\
                    Connection: close\n\
                    \n\
                    {}
                ", content.len(), content).to_string();

    result
}
