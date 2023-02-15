#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("qiri/include/qiri.hpp");
        fn connect(input: i32) -> i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = ffi::connect(42);
        assert_eq!(result, 42);
    }
}
