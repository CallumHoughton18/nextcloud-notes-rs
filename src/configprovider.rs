use std::path::PathBuf;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use serde::{Deserialize, Serialize};
use directories_next::{BaseDirs, UserDirs, ProjectDirs};
use std::fs;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct NxCloudNotesConfigData {
    pub server_address: String,
    pub base_notes_directory: String,
    pub user_name: String,
    pub password: Option<String>,
}

pub trait NxCloudConfigRetriever {
    fn has_config(self) -> bool;
    fn load_config(config_name: &str) -> Result<NxCloudNotesConfigData, Box<dyn Error>>;
    fn create_new_config(self, config: NxCloudNotesConfigData) -> Result<bool, Box<dyn Error>>;
}

pub struct FileSystemNxCloudConfig<'a> {
    config_name: &'a str
}

impl<'a> FileSystemNxCloudConfig<'a> {
    pub fn new(config_name: &'a str) -> Self {
        return Self {
            config_name
        }
    }
    pub fn get_config_dir(&self) -> Result<PathBuf, Box<dyn Error>> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "", "NxCloudNotes") {
            return Ok(proj_dirs.config_dir().to_owned())
        }

        Err("No valid home directory set for the system")?
        
    }
}

impl<'a> NxCloudConfigRetriever for FileSystemNxCloudConfig<'a> {
    fn has_config(self) -> bool {
        match &self.get_config_dir() {
            Ok(path) => {
                true
            },
            Err(e) => {
                false
            }
        }
    }
    
    fn load_config(config_name: &str) -> Result<NxCloudNotesConfigData, Box<dyn Error>> {
        todo!();
        // if let Some(proj_dirs) = ProjectDirs::from("com", "", "NxCloudNotes") {
        //     let config_dir = proj_dirs.config_dir();
        //     fs::create_dir_all(config_dir)?;

        // }
        // let config: NxCloudNotesConfigData = confy::load(config_name)?;
        // Ok(config)
    }

    fn create_new_config(self, config: NxCloudNotesConfigData) -> Result<bool, Box<dyn Error>> {
        let config_dir = &self.get_config_dir()?;
        if !config_dir.exists() {
            fs::create_dir(config_dir)?;
        }
        let toml = toml::to_string(&config)?;
        let config_path = config_dir.join(&self.config_name);
        println!("{}", config_path.to_str().unwrap());
        if config_path.exists() {
            fs::remove_file(&config_path)?;
        }
        let mut file = File::create(config_path)?;
        file.write_all(toml.as_bytes())?;
        Ok(true)
    }
}
