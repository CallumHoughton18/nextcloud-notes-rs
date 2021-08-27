use std::error::Error;
use crate::httprequest::HttpRequest;
use crate::httprequest::RequestType;


pub struct NextCloudClient<T> 
where 
    T: HttpRequest 
{
    request_builder: T,
    username: String,
    password: String,
}

impl<T: HttpRequest> NextCloudClient<T> {
    pub fn new (t: T, username: String, password: String ) -> Self {
        NextCloudClient {
            request_builder: t,
            username,
            password
        }
    }

    pub fn create_or_replace_file<'a>(self, filepath: &str, content: &[u8]) -> Result<&'a str, Box<dyn Error>> {
        let call_result = self.request_builder
        .set_request(RequestType::PUT, &format!("/remote.php/dav/files/{}{}", &self.username, filepath))
        .basic_auth(&self.username, &self.password)
        .set_header("OCS-APIRequest".to_string(), "true".to_string())
        .set_header("Connection".to_string(), "closed".to_string())
        .send_bytes(content)?;

        if (199..300).contains(&call_result.response_code) {
            Ok("File uploaded successfully")
        } else {
            Err(format!("Reponse code {} indicates failure uploading file:\r\n{}", call_result.response_code, call_result.response_msg))?
        }
    }
}