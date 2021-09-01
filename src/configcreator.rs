use std::io::Write;
use std::io::BufRead;
use crate::utils::get_user_input;
use crate::configprovider::NxCloudNotesConfigData;

pub fn ask_user_for_config<R, W>(reader: &mut R, writer: &mut W) -> NxCloudNotesConfigData
where 
    R: BufRead,
    W: Write,
     {
    let server_address = get_user_input(reader, writer, "Enter server address (ie nextcloud.myserver.net):", true).expect("Error getting server address input");
    let base_notes_directory = get_user_input(reader, writer, "Enter notes directory, this must exist on your NextCloud account:", true).expect("Error notes directory input");
    let user_name = get_user_input(reader, writer, "Enter your username:", true).expect("Error getting username input");
    let password = get_user_input(reader, writer, "Enter your password:", false).expect("Error getting password input");

    let password = if password.is_empty() {None} else {Some(password)};

    NxCloudNotesConfigData {
        server_address,
        base_notes_directory,
        user_name,
        password
    }
}