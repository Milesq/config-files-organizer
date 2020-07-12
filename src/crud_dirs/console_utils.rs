use std::io::{self, Read};

pub fn clear_console() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn get_char() -> io::Result<u8> {
    let mut buff: &mut [u8] = &mut [0u8];
    io::stdin().read(&mut buff)?;

    Ok(buff[0])
}
