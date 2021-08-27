use std::io::BufReader;
use std::error::Error;
use openssl::ssl::SslStream;
use std::collections::HashMap;
use std::net::{TcpStream};
use std::io::{Read, Write};
use openssl::ssl::{SslMethod, SslConnector};
use std::io::BufRead;

pub enum RequestType {
    GET,
    PUT
}

pub struct LiteHttpClient {
    base_address: String,
    port: u32,
    request_line: String,
    headers: HashMap<String, String>, 
}

impl LiteHttpClient {
    pub fn new(base_address: String, port: u32) -> Self {
        LiteHttpClient {
            base_address,
            port,
            request_line: "/".to_string(),
            headers: HashMap::new()
        }
    }
    fn connect_to_server(&self) -> Result<SslStream<TcpStream>,Box<dyn Error>> {
        let connector = SslConnector::builder(SslMethod::tls())?.build();
        let address_with_port = format!("{}:{}", self.base_address, self.port);

        if let Ok(stream) = TcpStream::connect(address_with_port) {
            let ssl_stream = connector.connect(&self.base_address, stream)?;
            Ok(ssl_stream)
        } else {
            Err("Unable to connect to given host")?
        }
    }

    fn parse_response(&self, stream: SslStream<TcpStream>) -> Result<HttpResponse, Box<dyn Error>> {
        let mut buf = String::new();
        let mut reader = BufReader::new(stream);
        let mut top_line = String::new();
        reader.read_line(&mut top_line)?;
        let response_code: u16 = top_line[9..12].parse().unwrap();
        reader.read_to_string(&mut buf)?;
        let mut splitter = buf.splitn(2, "\r\n\r\n");
        splitter.next().unwrap();
        let second = splitter.next().unwrap();
        Ok(HttpResponse {
            response_code,
            response_msg: second.to_string()
        }) 
    }
}

pub struct HttpResponse {
    pub response_code: u16,
    pub response_msg: String
}

pub trait HttpRequest {
    fn set_request(self, req_type: RequestType, remote_uri: &str) -> Self;
    fn set_header(self, header: String, value: String) -> Self;
    fn basic_auth(self, user: &str, password: &str) -> Self;
    fn send_bytes(self, bytes: &[u8]) -> Result<HttpResponse, Box<dyn Error>>;
}

impl HttpRequest for LiteHttpClient {
    fn set_request(mut self, req_type: RequestType, remote_uri: &str) -> Self {
        match req_type {
            RequestType::GET => {
                self.request_line = format!("GET {} HTTP/1.1\r\n", remote_uri);
            },
            RequestType::PUT => {
                self.request_line = format!("PUT {} HTTP/1.1\r\n", remote_uri);
            }
        };

        self.headers.insert("Host".to_string(), String::from(&self.base_address));
        self
    }

    fn set_header(mut self, header: String, value: String) -> Self {
        self.headers.insert(header, value);
        self
    }

    fn basic_auth(mut self, user: &str, password: &str) -> Self {
        let user_and_password = format!("{}:{}", user, password);
        let encoded = openssl::base64::encode_block(user_and_password.as_bytes());
        self.headers.insert(String::from("Authorization"), format!("Basic {}", encoded));
        self
    }

    fn send_bytes(mut self, bytes: &[u8]) -> Result<HttpResponse, Box<dyn Error>> {
        let mut stream = self.connect_to_server()?;

        let mut request_data = String::new();
        request_data.push_str(&self.request_line);

        self.headers.insert(String::from("Accept"), String::from("*/*"));
        self.headers.insert(String::from("Content-Length"), bytes.len().to_string());

        for (key, value) in self.headers.iter_mut() {
            let header_formatted = format!("{}: {}\r\n", key, value);
            request_data.push_str(&header_formatted)
        }
        request_data.push_str("\r\n");

        stream.write_all(request_data.as_bytes())?;
        stream.flush().unwrap();
        stream.write_all(bytes)?;

        self.parse_response(stream)
    }
}