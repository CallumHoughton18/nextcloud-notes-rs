use std::error::Error;
use std::io::{BufRead, Write};

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
        // Remove /n ending character
        buf.pop();
        if !buf.is_empty() || !is_required {
            break;
        }
    }
    Ok(buf)
}
