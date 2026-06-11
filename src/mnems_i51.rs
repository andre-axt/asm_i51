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
    Mov(Operand, Operand),
    Movc(Operand, Operand),
    Movx(Operand, Operand),
    Push(Operand),
    Pop(Operand),
    Xch(Operand, Operand),
    Xchd(Operand, Operand),
    Add(Operand, Operand),
    Addc(Operand, Operand),
    Subb(Operand, Operand),
    Inc(Operand),
    Dec(Operand),
    Da(Operand),
    Anl(Operand, Operand),
    Orl(Operand),
    Xrl(Operand),
    Clr(Operand),
    Cpl(Operand),
    Rl(Operand),
    Rlc(Operand),
    Rr(Operand),
    Rrc(Operand),
    Swap(Operand),
    Setb(Operand),
    Jc(Operand),
    Jnc(Operand),
    Jb(Operand, Operand),
    Jnb(Operand, Operand),
    Jbc(Operand, Operand),
    Acall(Operand),
    Lcall(Operand),
    Ret,
    Reti,
    Ajmp(Operand),
    Ljmp(Operand),
    Sjmp(Operand),
    Jmp(Operand),
    Jz(Operand),
    Jnz(Operand),
    Cjne(Operand, Operand, Operand),
    Djnz(Operand, Operand),
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
