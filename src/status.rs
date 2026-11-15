// #[repr(u8)]
// #[derive(Copy, Clone)]
pub enum Status {
    InsufficientArgs = 2,
    InvalidLanguage = 3,
    NoName = 4,
    CreationFailure = 5,
    WriteFailure = 6,
}
