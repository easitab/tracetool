use std::io::Read;

/// Read the contents of stdin into a string. This function will block until EOF is reached.
pub(crate) fn read_stdin_string() -> crate::util::Result<String> {
    let mut buffer = String::new();
    match std::io::stdin().read_to_string(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(e) => Err(e.into()),
    }
}
