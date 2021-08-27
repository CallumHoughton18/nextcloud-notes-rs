use std::io::Read;
use std::fs::File;
use nxcloud_notes_rs::httprequest::LiteHttpClient;
use nxcloud_notes_rs::httprequest::HttpRequest;
use nxcloud_notes_rs::httprequest::RequestType;

fn main() {
    // TODO: implement actual credentials provider
    let mut config_file_contents = String::new();
    File::open("config.txt").unwrap().read_to_string(&mut config_file_contents).unwrap();
    let mut splitter = config_file_contents.splitn(2, ",");
    let username = splitter.next().unwrap();
    let password = splitter.next().unwrap();
    println!("{}: {}", username, password);

    let http_client = LiteHttpClient::new("storage.callums-stuff.net".to_string(), 443, "/remote.php/dav/files/CallumHoughton22/Notes/testfile.txt".to_string());
    let call_result = http_client
    .set_request(RequestType::PUT, "/remote.php/dav/files/CallumHoughton22/Notes/testfile.txt")
    .basic_auth(&username, &password)
    .set_header("OCS-APIRequest".to_string(), "true".to_string())
    .set_header("Connection".to_string(), "closed".to_string())
    .send_bytes(b"hello world!");

    match call_result {
        Ok(res) => {
            println!("code: {} \r\nresponse: {}", res.response_code, res.response_msg);
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}
