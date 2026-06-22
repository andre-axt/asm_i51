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
#[derive(Debug)]
pub struct Command {
    opcode: String,
    operand1: Option<String>,
    operand2: Option<String>,
}

pub fn find_command(buffer: &mut String) -> Option<Command> {
    let delims = ['\n', '#', ';'];
    let pos = buffer.find(|c: char| delims.contains(&c))?;
    let prefix = buffer[..pos].trim().to_string();

    if let Some(delim) = buffer[pos..].chars().next() {
        let delim_len = delim.len_utf8();
        let remove_until = pos + delim_len;
        buffer.drain(..remove_until);
    }

    if prefix.is_empty() {
        return None;
    }

    let parts: Vec<&str> = prefix.split_whitespace().collect();
    let opcode = parts.first()?.to_string();
    
    let operand1 = parts.get(1).map(|s| s.to_string());
    let operand2 = parts.get(2).map(|s| s.to_string());

    Some(Command {
        opcode,
        operand1,
        operand2,
    })
}
