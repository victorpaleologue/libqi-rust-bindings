use core::fmt;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("qiri/include/qiri.hpp");

        type Session;
        fn new_session() -> UniquePtr<Session>;
        fn session_listen(s: &UniquePtr<Session>, endpoint: String);
        fn connect(endpoint: String) -> UniquePtr<Session>;
    }
}

impl fmt::Debug for ffi::Session {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Qi Session")
    }
}

#[cfg(test)]
mod tests {
    use super::ffi::*;

    #[test]
    fn can_create_a_session() {
        let session = new_session();
        assert!(!session.is_null());
    }

    #[test]
    fn session_can_connect() {
        let server = new_session();
        assert!(!server.is_null());
        let endpoint = "tcp://127.0.0.1:12345".to_string();
        session_listen(&server, endpoint.to_owned());
        let client = connect(endpoint);
        assert!(!client.is_null());
    }
}
