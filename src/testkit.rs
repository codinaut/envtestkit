use std::env;
use std::ffi::{OsStr, OsString};

pub struct EnvironmentTestGuard {}

pub fn set_env<V: AsRef<OsStr>>(key: OsString, value: V) {
    env::set_var(key, value)
}

#[cfg(test)]
mod test {
    use super::*;
    use fake::faker::lorem::en::*;
    use fake::Fake;

    #[test]
    fn set_env_ok() {
        let key = OsString::from(Word().fake::<String>());
        let value = OsString::from(Sentence(1..10).fake::<String>());

        set_env(key.clone(), &value);
        assert_eq!(env::var_os(key).unwrap(), value)
    }
}
