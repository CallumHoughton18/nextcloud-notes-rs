use directories_next::ProjectDirs;
use nxcloud_notes_rs::configcreator::ask_user_for_config;
use nxcloud_notes_rs::configprovider::NxCloudConfigRetriever;
use nxcloud_notes_rs::configprovider::FileSystemNxCloudConfig;
use nxcloud_notes_rs::httprequest::LiteHttpClient;
use nxcloud_notes_rs::nextcloudclient::NextCloudClient;
use std::io::{self};

fn main() {
    let config_project_dir = ProjectDirs::from("com", "", "NxCloudNotes")
    .expect("No valid home directory set for the system. Config cannot be saved. App exiting...");
    let config_dir_path = config_project_dir.config_dir().to_owned();
    let config_provider = FileSystemNxCloudConfig::new(&config_dir_path, "test-config.toml");

    if !config_provider.has_config().expect("An error occurred retrieving user config") {
        let stdio = io::stdin();
        let mut input = stdio.lock();
        let mut output = io::stdout(); 
           
        let inputted_config = ask_user_for_config(&mut input, &mut output);
        config_provider.create_new_config(inputted_config).unwrap();
    }

    let config_data = config_provider.load_config().unwrap();
    println!("{:?}", config_data);
 
    let http_client = LiteHttpClient::new(config_data.server_address, 443);
    let nextcloud_client = NextCloudClient::new(http_client, config_data.user_name, config_data.password.unwrap());
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
