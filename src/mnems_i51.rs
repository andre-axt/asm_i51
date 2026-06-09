#[derive(Debug, PartialEq)]
pub enum Mnems {
    Nop,
    Ajmp(u16),
    Ljmp(u8, u8, u8),
    Rr(u8),
    Inc(u8),
    Inc_Direct(u16),
    Jbc(u8, u8, u8),

}

pub fn find_command(buffer: &mut String) -> Option<String> {
    let delims = ['\n', '#', ';'];
    let pos = buffer.find(delims)?;
    
    let prefix = buffer[..pos].to_string();
    
    let delims = &buffer[pos..].chars().next().unwrap();
    let delims_len = delims.len_utf8();
    let remove_until = pos + delims_len;
    
    buffer.drain(..remove_until);
    
    Some(prefix)
}
