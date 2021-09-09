use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;
use std::vec::IntoIter;


const USAGE: &'static str = "
nxcloudnotes -- A NextCloud CLI dumping tool.
Usage:
  nxcloudnotes <command> [<args>...]
  nxcloudnotes help
  nxcloudnotes config-path
  nxcloudnotes \"NOTES BODY\"
  nxcloudnotes -p\"NxCloudPassword\" \"NOTES BODY\"
  nxcloudnotes -t\"NOTES TITLE\" \"NOTES BODY\"
Commands:
  help             Display usage information.
  config-path      Output path to .toml config file used for this application.
  \"\"             Empty strings are treated as the notes body if no other commands are found.
";


#[derive(Debug, PartialEq)]
pub enum ProgramCommands {
    PostNote(PostNoteCLIConfig),
    ConfigPath,
    Help(&'static str),
}

// for internal use only, this is just the content parsed from the given arguments on the command line
// what should be returned to the user is the ProgramCommands enum
enum ParsedCommands {
    PostNote(String),
    ConfigPath,
    Help
}

#[derive(Debug, PartialEq)]
pub struct PostNoteCLIConfig {
    pub password: Option<String>,
    pub title: Option<String>,
    pub content: String,
}

#[derive(Debug)]
enum Arg<FlagType> {
    Plain(FlagType),
    Short(FlagType, String),
}

impl Arg<String> {
    fn flag_as_ref(&self) -> Arg<&str> {
        match *self {
            Arg::Plain(ref x) => Arg::Plain(&x[..]),
            Arg::Short(ref x, ref value) => Arg::Short(&x[..], value.clone()),
        }
    }
}

impl Display for Arg<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match *self {
            Arg::Short(ref arg, ref _val) => write!(f, "-{}", arg),
            Arg::Plain(ref cmd) => write!(f, "{}", cmd),
        }
    }
}

struct ArgIter {
    args: IntoIter<String>,
}

impl ArgIter {
    pub fn new(args: Vec<String>) -> ArgIter {
        ArgIter {
            args: args.into_iter(),
        }
    }
}

impl Iterator for ArgIter {
    type Item = Arg<String>;

    fn next(&mut self) -> Option<Arg<String>> {
        let arg = self.args.next()?;

        if arg.starts_with("-") {
            let mut flag = String::from(&arg[1..]);
            if flag.len() > 1 {
                let user_value = flag.split_off(1);
                flag.truncate(1);
                return Some(Arg::Short(flag, user_value));
            }
        }

        Some(Arg::Plain(arg))
    }
}

pub fn parse_args(argv: Vec<String>) -> Result<ProgramCommands, String> {
    let mut args = ArgIter::new(argv);
    // Skip the executable name
    args.next();

    let mut operator = ParsedCommands::Help;
    // each supported flag is placed into a hashmap where it can be used
    let mut flag_map = HashMap::new();

    while let Some(arg) = args.next() {
        match arg.flag_as_ref() {
            Arg::Plain("config-path") => operator = ParsedCommands::ConfigPath,
            Arg::Plain("help") => operator = ParsedCommands::Help,
            Arg::Plain(any_other_val) => operator = ParsedCommands::PostNote(String::from(any_other_val)),
            Arg::Short(flag, value) => {flag_map.insert(String::from(flag), value);}
        }
    }

    let cmd = match operator {
        ParsedCommands::ConfigPath => ProgramCommands::ConfigPath,
        ParsedCommands::Help => ProgramCommands::Help(USAGE),
        ParsedCommands::PostNote(content) => ProgramCommands::PostNote(parse_flags_to_post_note_cli_config(flag_map, content))
    };

    Ok(cmd)
}

fn parse_flags_to_post_note_cli_config(mut flag_map: HashMap<String, String>, content: String) -> PostNoteCLIConfig {
    PostNoteCLIConfig {
        title: flag_map.remove("t"),
        password: flag_map.remove("p"),
        content
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_slice(args: &[&'static str]) -> Result<ProgramCommands, String> {
        let argv = args.iter().map(|s| String::from(*s)).collect();
        parse_args(argv)
    }
    
    #[test]
    fn should_parse_help_command_correctly() {
        let help_command = Ok(ProgramCommands::Help);
        assert_eq!(parse_slice(&["nxcloudnotes", "help"]), help_command);
        assert_eq!(parse_slice(&["nxcloudnotes", "test", "bar" ,"help", "-ttestparams"]), help_command);
        assert_eq!(parse_slice(&["nxcloudnotes", "-t\"testparams\"", "help"]), help_command)
    }

    #[test]
    fn should_parse_config_path_command_correctly() {
        let config_path_command = Ok(ProgramCommands::ConfigPath);
        assert_eq!(parse_slice(&["nxcloudnotes", "config-path"]), config_path_command);
        assert_eq!(parse_slice(&["nxcloudnotes", "test", "bar" ,"config-path", "-ttestparams"]), config_path_command);
        assert_eq!(parse_slice(&["nxcloudnotes", "-ttestparams", "config-path"]), config_path_command)
    }

    #[test]
    fn should_parse_post_note_command_correctly() {
        let post_note_command = Ok(ProgramCommands::PostNote(PostNoteCLIConfig{
            password: None,
            title: None,
            content: "note content here".to_string()
        }));
        assert_eq!(parse_slice(&["nxcloudnotes", "note content here"]), post_note_command);
    }  

    #[test]
    fn should_parse_post_note_command_with_flags() {
        let post_note_command = Ok(ProgramCommands::PostNote(PostNoteCLIConfig{
            password: Some("password".to_string()),
            title: Some("title".to_string()),
            content: String::from("note content here")
        }));
        assert_eq!(parse_slice(&["nxcloudnotes", "note content here", "-ttitle", "-ppassword"]), post_note_command);   
        assert_eq!(parse_slice(&["nxcloudnotes", "-ttitle", "-ppassword", "note content here"]), post_note_command);   
     }
}
