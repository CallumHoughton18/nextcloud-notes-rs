use nxcloud_notes_rs::configprovider::NxCloudNotesConfigData;
use nxcloud_notes_rs::configprovider::NxCloudConfigRetriever;
use nxcloud_notes_rs::configprovider::FileSystemNxCloudConfig;
use std::io::Read;
use std::fs::File;
use nxcloud_notes_rs::httprequest::LiteHttpClient;
use nxcloud_notes_rs::nextcloudclient::NextCloudClient;

fn main() {
    // TODO: implement actual credentials provider
    let mut config_file_contents = String::new();
    File::open("config.txt").unwrap().read_to_string(&mut config_file_contents).unwrap();
    let mut splitter = config_file_contents.splitn(2, ",");
    let username = splitter.next().unwrap();
    let password = splitter.next().unwrap();

    let config_provider = FileSystemNxCloudConfig::new("test-config.toml");
    config_provider.create_new_config(NxCloudNotesConfigData {
        server_address: "test".to_string(),
        base_notes_directory: "/test".to_string(),
        user_name: "test".to_string(),
        password: Some("TestPassword".to_string())
    }).unwrap();
 
    let http_client = LiteHttpClient::new("storage.callums-stuff.net".to_string(), 443);
    let nextcloud_client = NextCloudClient::new(http_client, username.to_string(), password.to_string());
    let result = nextcloud_client.create_or_replace_file("/Notes/testfile.txt", b"Hello front nextcloud client!");

    match result {
        Ok(msg) => {
            println!("success: {}", msg);
        },
        Err(e) => {
            println!("oopsy!: {}", e);
        }
    }
}
