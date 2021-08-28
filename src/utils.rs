use std::error::Error;
use std::io;

pub fn get_user_input(request_msg: &str, is_required: bool) -> Result<String, Box<dyn Error>> {
    let mut buf = String::new();
    let field_type = if is_required { "REQUIRED" } else { "OPTIONAL" };
    loop {
        println!("\r\n{}: {}", field_type, request_msg);
        io::stdin()
            .read_line(&mut buf)?;
        // Remove /n ending character
        buf.pop();
        if !buf.is_empty() || !is_required {
            break;
        }
    }
    Ok(buf)
}
