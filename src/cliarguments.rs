use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;
use std::vec::IntoIter;

#[derive(Debug)]
pub enum Cmd {
    PostNote(PostNoteCLIConfig),
    ConfigPath(),
    Help(),
}

enum Operator {
    PostNote,
    ConfigPath,
    Help
}

#[derive(Debug)]
pub struct PostNoteCLIConfig {
    password: Option<String>,
    title: Option<String>,
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
            Arg::Plain(_any_other_val) => operator = Operator::PostNote,
            Arg::Short(flag, value) => {flag_map.insert(String::from(flag), value);}
            _ => return unexpected(arg),
        }
    }

    let cmd = match operator {
        Operator::ConfigPath => Cmd::ConfigPath(),
        Operator::Help => Cmd::Help(),
        Operator::PostNote => Cmd::PostNote(parse_flags_to_post_note_cli_config(flag_map))
    };

    println!("{:?}", cmd);
    Ok(cmd)
}

fn parse_flags_to_post_note_cli_config(mut flag_map: HashMap<String, String>) -> PostNoteCLIConfig {
    PostNoteCLIConfig {
        title: flag_map.remove("t"),
        password: flag_map.remove("p")
    }

}

fn unexpected<T>(arg: Arg<String>) -> Result<T, String> {
    Err(format!("Unexpected argument {}. See 'tako --help'.", arg))
}
