use nxcloudnotes::cliarguments::PostNoteCLIConfig;
use directories_next::ProjectDirs;
use nxcloudnotes::configcreator::ask_user_for_config;
use nxcloudnotes::configprovider::NxCloudConfigRetriever;
use nxcloudnotes::configprovider::FileSystemNxCloudConfig;
use nxcloudnotes::httprequest::LiteHttpClient;
use nxcloudnotes::nextcloudclient::NextCloudClient;
use nxcloudnotes::cliarguments;
use std::io::{self};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let pattern: Vec<String> = std::env::args().collect();
    let command = cliarguments::parse_args(pattern).unwrap();

    let config_project_dir = ProjectDirs::from("com", "", "NxCloudNotes")
    .expect("No valid home directory set for the system. Config cannot be saved. App exiting...");
    let config_dir_path = config_project_dir.config_dir().to_owned();
    let config_provider = FileSystemNxCloudConfig::new(&config_dir_path, "app-config.toml");

    if !config_provider.has_config().expect("An error occurred retrieving user config") {
        let stdio = io::stdin();
        let mut input = stdio.lock();
        let mut output = io::stdout(); 
        let inputted_config = ask_user_for_config(&mut input, &mut output);
        config_provider.create_new_config(inputted_config).unwrap();
    }

    match command {
        cliarguments::ProgramCommands::Help(help_text) => {
            println!("{}", help_text)
        },

        cliarguments::ProgramCommands::ConfigPath => {
            let config_path_str = config_provider.config_path.into_os_string().into_string()
            .expect("Error getting config path");

            println!("config path: {}", config_path_str)
        },
        cliarguments::ProgramCommands::PostNote(cli_config) => {
            handle_post_note_command(config_provider, cli_config)
        }
    }
}

fn handle_post_note_command(config_provider: FileSystemNxCloudConfig, cli_config: PostNoteCLIConfig) {
    let config_data = config_provider.load_config().unwrap();

    let password = match config_data.password {
        Some(password_string) => password_string,
        None => cli_config.password.expect("You must provide a password with the -p argument if none is present in the applications config file"),
    };
 
    let title = match cli_config.title {
        Some(title_string) => title_string,
        None => {
            let start = SystemTime::now();
            let unix_time_since_epoch = start
                .duration_since(UNIX_EPOCH)
                .expect("Unable to generate unix timestamp for note title");
            unix_time_since_epoch.as_millis().to_string()
        }
    };
    let note_path = format!("/{}/{}.txt", config_data.base_notes_directory, title).to_string();
    let http_client = LiteHttpClient::new(config_data.server_address, config_data.port);
    let nextcloud_client = NextCloudClient::new(http_client, config_data.user_name, password);
    let result = nextcloud_client.create_or_replace_file(&note_path, cli_config.content.as_bytes());

    match result {
        Ok(_) => {
            println!("successfully uploaded note");
        },
        Err(e) => {
            // Would be better to return a customized error with a friendly message here.
            println!("an error occured! {}", e);
        }
    }
}
