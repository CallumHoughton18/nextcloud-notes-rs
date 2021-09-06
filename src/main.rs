use directories_next::ProjectDirs;
use nxcloud_notes_rs::configcreator::ask_user_for_config;
use nxcloud_notes_rs::configprovider::NxCloudConfigRetriever;
use nxcloud_notes_rs::configprovider::FileSystemNxCloudConfig;
use nxcloud_notes_rs::httprequest::LiteHttpClient;
use nxcloud_notes_rs::nextcloudclient::NextCloudClient;
use nxcloud_notes_rs::cliarguments;
use std::io::{self};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let pattern: Vec<String> = std::env::args().collect();
    let command = cliarguments::parse_args(pattern).unwrap();

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

    match command {
        cliarguments::ProgramCommands::Help => {
            todo!()
        },

        cliarguments::ProgramCommands::ConfigPath => {
            todo!()
        },
        cliarguments::ProgramCommands::PostNote(cli_config) => {
            let config_data = config_provider.load_config().unwrap();
            println!("{:?}", config_data);

            let password = match config_data.password {
                Some(password_string) => password_string,
                None => cli_config.password.expect("You must provide a password with the -p argument if none is present in the programs config file"),
            };
         
            let title = match cli_config.title {
                Some(title_string) => title_string,
                None => {
                    let start = SystemTime::now();
                    let unix_time_since_epoch = start
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards");
                    unix_time_since_epoch.as_millis().to_string()
                }
            };
            let note_path = format!("/{}/{}.txt", config_data.base_notes_directory, title).to_string();
            let http_client = LiteHttpClient::new(config_data.server_address, config_data.port);
            let nextcloud_client = NextCloudClient::new(http_client, config_data.user_name, password);
            let result = nextcloud_client.create_or_replace_file(&note_path, cli_config.content.as_bytes());
        
            match result {
                Ok(msg) => {
                    println!("success: {}", msg);
                },
                Err(e) => {
                    println!("oopsy!: {}", e);
                }
            }
        }
    }
}
