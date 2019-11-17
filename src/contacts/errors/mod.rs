#[derive(PartialEq, Eq, Debug)]
pub enum Error {
    InvalidEmail,
    EmailIsMissing,
    DuplicateContact,
}