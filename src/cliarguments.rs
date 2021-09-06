use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;
use std::vec::IntoIter;

#[derive(Debug, PartialEq)]
pub enum Cmd {
    PostNote(PostNoteCLIConfig),
    ConfigPath(),
    Help(),
}

enum Operator {
    PostNote(String),
    ConfigPath,
    Help
}

#[derive(Debug, PartialEq)]
pub struct PostNoteCLIConfig {
    password: Option<String>,
    title: Option<String>,
    content: String,
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

pub fn parse_args(argv: Vec<String>) -> Result<Cmd, String> {
    let mut args = ArgIter::new(argv);
    // Skip the executable name
    args.next();

    let mut operator = Operator::Help;
    let mut flag_map = HashMap::new();

    while let Some(arg) = args.next() {
        match arg.flag_as_ref() {
            Arg::Plain("config-path") => operator = Operator::ConfigPath,
            Arg::Plain("help") => operator = Operator::Help,
            Arg::Plain(any_other_val) => operator = Operator::PostNote(String::from(any_other_val)),
            Arg::Short(flag, value) => {flag_map.insert(String::from(flag), value);}
            _ => return unexpected(arg),
        }
    }

    let cmd = match operator {
        Operator::ConfigPath => Cmd::ConfigPath(),
        Operator::Help => Cmd::Help(),
        Operator::PostNote(content) => Cmd::PostNote(parse_flags_to_post_note_cli_config(flag_map, content))
    };

    println!("{:?}", cmd);
    Ok(cmd)
}

fn parse_flags_to_post_note_cli_config(mut flag_map: HashMap<String, String>, content: String) -> PostNoteCLIConfig {
    PostNoteCLIConfig {
        title: flag_map.remove("t"),
        password: flag_map.remove("p"),
        content
    }

}

fn unexpected<T>(arg: Arg<String>) -> Result<T, String> {
    Err(format!("Unexpected argument {}. See 'tako --help'.", arg))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_slice(args: &[&'static str]) -> Result<Cmd, String> {
        let argv = args.iter().map(|s| String::from(*s)).collect();
        parse_args(argv)
    }
    
    #[test]
    fn should_parse_help_command_correctly() {
        let help_command = Ok(Cmd::Help());
        assert_eq!(parse_slice(&["nxcloudnotes", "help"]), help_command);
        assert_eq!(parse_slice(&["nxcloudnotes", "test", "bar" ,"help", "-ttestparams"]), help_command);
        assert_eq!(parse_slice(&["nxcloudnotes", "-t\"testparams\"", "help"]), help_command)
    }

    #[test]
    fn should_parse_config_path_command_correctly() {
        let config_path_command = Ok(Cmd::ConfigPath());
        assert_eq!(parse_slice(&["nxcloudnotes", "config-path"]), config_path_command);
        assert_eq!(parse_slice(&["nxcloudnotes", "test", "bar" ,"config-path", "-ttestparams"]), config_path_command);
        assert_eq!(parse_slice(&["nxcloudnotes", "-ttestparams", "config-path"]), config_path_command)
    }

    #[test]
    fn should_parse_post_note_command_correctly() {
        let post_note_command = Ok(Cmd::PostNote(PostNoteCLIConfig{
            password: None,
            title: None,
            content: "note content here".to_string()
        }));
        assert_eq!(parse_slice(&["nxcloudnotes", "note content here"]), post_note_command);
    }  

    #[test]
    fn should_parse_post_note_command_with_flags() {
        let post_note_command = Ok(Cmd::PostNote(PostNoteCLIConfig{
            password: Some("password".to_string()),
            title: Some("title".to_string()),
            content: String::from("note content here")
        }));
        assert_eq!(parse_slice(&["nxcloudnotes", "note content here", "-ttitle", "-ppassword"]), post_note_command);   
        assert_eq!(parse_slice(&["nxcloudnotes", "-ttitle", "-ppassword", "note content here"]), post_note_command);   
     }
}