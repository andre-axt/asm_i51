#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Accumulator,
    Register(u8),
    Direct(u8),
    Immediate(u8),
    Bit(u8),
    Indirect(u8),
    Relative(i8),
    CodeAddr(u16),
}
#[derive(Debug, Clone, PartialEq)]
pub enum Mnems {
    Nop,
    Ajmp(u16),
    Ljmp(u8, u8, u8),
    Rr(u8),
    Inc(u8),
    Inc_Direct(u16),
    Jbc(u8, u8, u8),
    Dec_A(u8),
    Dec_Rn(u8),
    Dec_Direct(u16),
    Mul(u8),
    Div(u8),
    Da(u8),

}

pub struct Command {
    opcode: String,
    operand1: Option<String>,
    operand2: Option<String>,
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
