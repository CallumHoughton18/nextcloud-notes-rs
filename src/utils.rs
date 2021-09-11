use std::error::Error;
use std::io::{BufRead, Write};

/// Gets the user input from the given `reader` BufRead buffer.
/// Which will loop if the input is empty and `is_required` is true.
pub fn get_user_input<R, W>(
    reader: &mut R,
    mut writer: &mut W,
    request_msg: &str,
    is_required: bool,
) -> Result<String, Box<dyn Error>>
where
    R: BufRead,
    W: Write,
{
    let mut buf = String::new();
    let field_type = if is_required { "REQUIRED" } else { "OPTIONAL" };
    loop {
        writeln!(&mut writer, "\r\n{}: {}", field_type, request_msg)?;
        reader.read_line(&mut buf)?;
        trim_newline_characters(&mut buf);
        if !buf.is_empty() || !is_required {
            break;
        }
    }
    Ok(buf)
}

/// Trims any \r and \n characters from the `s` argument.
fn trim_newline_characters(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

