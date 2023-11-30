use super::encryption::FileCrypt;
use anyhow::{Ok, Result};
use std::{
    fs::{File, OpenOptions},
    io::Read,
    io::Write,
};

pub fn write_contents_to_file(file: &str, contents: Vec<u8>) -> Result<()> {
    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(file)?;
    f.write_all(contents.as_slice())
        .expect("failed writing to file");
    Ok(f.flush()?)
}

//TODO: get rid of !!! ?
pub fn prepend_uuid(uuid: &String, encrypted_contents: &mut Vec<u8>) -> Vec<u8> {
    let mut uuid_bytes = uuid.as_bytes().to_vec();
    let mut tmp: Vec<u8> = vec![33, 33, 33];
    uuid_bytes.append(&mut tmp);
    uuid_bytes.append(encrypted_contents);
    uuid_bytes
}
