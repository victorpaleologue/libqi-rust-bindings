mod bridge;
pub use bridge::ffi::*;
use core::fmt;

impl fmt::Debug for Session {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Qi Session")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_a_session() {
        let session = new_session();
        assert!(!session.is_null());
    }

    #[test]
    fn session_can_connect() {
        let server = new_session();
        let endpoint = "tcp://127.0.0.1:12345".to_string();
        session_listen(&server, endpoint.to_owned());

        let client = new_session();
        session_connect(&client, endpoint.to_owned());
    }
}
