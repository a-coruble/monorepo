use std::{
    fs::{File, OpenOptions},
    path::Path,
};

pub const CONFIGURATION_FILE_NAME: &str = ".aliasmanagerrc";

pub enum AliasmanagerFilePermissions {
    Append,
    Read,
    Write,
}

pub fn open_file_with_permissions(
    path: &Path,
    permissions: AliasmanagerFilePermissions,
) -> Result<File, String> {
    let mut opener = OpenOptions::new();
    opener.read(true);
    if !path.exists() {
        opener.create(true);
    }
    match permissions {
        AliasmanagerFilePermissions::Append => opener.append(true),
        AliasmanagerFilePermissions::Read => opener.read(true),
        AliasmanagerFilePermissions::Write => opener.write(true),
    };
    opener
        .open(path)
        .map_err(|err| format!("Couldn't open file with underlying error: \n{}", err))
}
