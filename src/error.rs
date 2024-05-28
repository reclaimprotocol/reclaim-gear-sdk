use gstd::prelude::*;

#[derive(Debug)]
pub enum ContractError {
    Unauthorized {},
    SignatureErr {},
    NotFoundErr {},
}
