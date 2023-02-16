#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("qiri/include/qiri.hpp");

        type Session;
        fn new_session() -> UniquePtr<Session>;
        fn session_listen(s: &UniquePtr<Session>, endpoint: String);
        fn session_connect(s: &UniquePtr<Session>, endpoint: String);
    }
}