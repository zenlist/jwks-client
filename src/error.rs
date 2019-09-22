#[derive(Debug, PartialEq)]
pub struct Error {
    /// Debug message associated with error
    pub msg: &'static str,
    pub typ: Type,
}

/// Type of error encountered
#[derive(Debug, PartialEq)]
pub enum Type {
    /// Token is invalid (TODO: Remove this--too generic)
    Invalid,
    /// Token has expired
    Expired,
    /// Not Before (nbf) is set and it's too early to use the token
    Early,
    /// Problem with certificate
    Certificate,
    /// Problem with key
    Key,
    /// Could not download key set
    Connection,
    /// Problem with JWT header
    Header,
    /// Problem with JWT payload
    Payload,
    /// Problem with JWT signature
    Signature,
    /// Internal problem (Signals a serious bug or fatal error)
    Internal,
}

pub fn err(msg: &'static str, typ: Type) -> Error {
    Error {
        msg,
        typ
    }
}

pub fn err_inv(msg: &'static str) -> Error {
    err(msg, Type::Invalid)
}

pub fn err_exp(msg: &'static str) -> Error {
    err(msg, Type::Expired)
}

pub fn err_nbf(msg: &'static str) -> Error {
    err(msg, Type::Early)
}

pub fn err_cer(msg: &'static str) -> Error {
    err(msg, Type::Certificate)
}

pub fn err_key(msg: &'static str) -> Error {
    err(msg, Type::Key)
}

pub fn err_con(msg: &'static str) -> Error {
    err(msg, Type::Connection)
}

pub fn err_hea(msg: &'static str) -> Error {
    err(msg, Type::Header)
}

pub fn err_pay(msg: &'static str) -> Error {
    err(msg, Type::Payload)
}

pub fn err_sig(msg: &'static str) -> Error {
    err(msg, Type::Signature)
}

pub fn err_int(msg: &'static str) -> Error {
    err(msg, Type::Internal)
}

#[cfg(test)]
mod tests {

}
