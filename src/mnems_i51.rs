#[derive(Debug, PartialEq)]
pub enum mnems {
    Nop,
    Ajmp(u16),
    Ljmp(u8, u8, u8),
    Rr(u8),
    Inc(u8),
    Inc_Direct(u16),
    Jbc(u8, u8, u8),

}
