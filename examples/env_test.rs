use std::env;
use std::ffi::OsString;

fn main() {
    println!("Name: {}", get_name().to_str().unwrap());
}

fn get_name() -> OsString {
    match env::var_os("NAME") {
        Some(value) => value,
        None => OsString::from("default-value"),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use envtestkit::lock::{lock_read, lock_test};
    use envtestkit::set_env;
    use std::{thread, time};

    #[test]
    fn get_name_default() {
        let _lock = lock_read();
        assert_eq!(get_name(), "default-value")
    }

    #[test]
    fn get_name_overriden() {
        let _lock = lock_test();
        let _test = set_env(OsString::from("NAME"), "not-default-one");
        assert_eq!(get_name(), "not-default-one")
    }

    #[test]
    #[ignore]
    fn get_name_racey() {
        thread::sleep(time::Duration::from_millis(100));
        assert_eq!(get_name(), "default-value")
    }

    #[test]
    #[ignore]
    fn get_name_racey_inducer() {
        let _test = set_env(OsString::from("NAME"), "not-default-one");

        thread::sleep(time::Duration::from_millis(200));
        assert_eq!(get_name(), "not-default-one")
    }
}
