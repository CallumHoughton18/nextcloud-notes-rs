use crate::configprovider::NxCloudNotesConfigData;
use crate::utils::get_user_input;
use std::io::{BufRead, Write};

pub fn ask_user_for_config<R, W>(reader: &mut R, writer: &mut W) -> NxCloudNotesConfigData
where
    R: BufRead,
    W: Write,
{
    let server_address = get_user_input(
        reader,
        writer,
        "Enter server address WITH the port (ie nextcloud.myserver.net:443):",
        true,
    )
    .expect("Error getting server address input");

    let port = loop {
        let port = get_user_input(reader, writer, "Enter port number: ", true)
        .expect("Error getting port number input");
        let port_int = port.parse::<u32>();
        match port_int {
            Ok(port_as_int) => break port_as_int,
            Err(_) => {}
        }
    };

    let base_notes_directory = get_user_input(
        reader,
        writer,
        "Enter notes directory, this must exist on your NextCloud account:",
        true,
    )
    .expect("Error notes directory input");

    let user_name = get_user_input(reader, writer, "Enter your username:", true)
        .expect("Error getting username input");

    let password = get_user_input(reader, writer, "Enter your password:", false)
        .expect("Error getting password input");

    let password = if password.is_empty() {
        None
    } else {
        Some(password)
    }; 

    NxCloudNotesConfigData {
        server_address,
        port,
        base_notes_directory,
        user_name,
        password,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn should_parse_input_to_correct_config() {
        let input = Cursor::new("Test.storage.net\n443\nNotes\nTest User\nTestpassword\n".as_bytes());
      
        assert_config_is_correct(input, "Test.storage.net", 443, "Notes", "Test User", Some("Testpassword"))
    }

    #[test]
    fn should_parse_input_to_correct_config_with_carriage_returns() {
        let input = Cursor::new("Test.storage.net\r\n443\r\nNotes\r\nTest User\r\nTestpassword\r\n".as_bytes());
      
        assert_config_is_correct(input, "Test.storage.net", 443, "Notes", "Test User", Some("Testpassword"))
    }

    #[test]
    fn should_parse_input_to_config_with_none_password() {
        let input = Cursor::new("Test.storage.net\n443\nNotes\nTest User\n".as_bytes());
      
        assert_config_is_correct(input, "Test.storage.net", 443, "Notes", "Test User", None)
    }

    #[test]
    fn should_handle_incorrect_port_input() {
        // should handle the 'invalidPort' input and discaord it, but parse 443 correctly
        let input = Cursor::new("Test.storage.net\ninvalidPort\n443\nNotes\nTest User\n".as_bytes());
      
        assert_config_is_correct(input, "Test.storage.net", 443, "Notes", "Test User", None)
    }

    fn assert_config_is_correct(
        mut input: Cursor<&[u8]>,
        expected_server_address: &str,
        expected_port: u32,
        expected_base_notes_directory: &str,
        expected_user_name: &str,
        expected_password: Option<&str>,
    ) {
        let mut output = Vec::new();

        let actual_config = ask_user_for_config(&mut input, &mut output);

        assert_eq!(&actual_config.server_address, expected_server_address);
        assert_eq!(actual_config.port, expected_port);
        assert_eq!(&actual_config.base_notes_directory, expected_base_notes_directory);
        assert_eq!(&actual_config.user_name, expected_user_name);

        match expected_password {
            Some(expected_password_str) => {
                assert_eq!(&actual_config.password.unwrap(), expected_password_str);
            },
            None => {
                assert!(&actual_config.password.is_none());
            }
        }
    }
}
