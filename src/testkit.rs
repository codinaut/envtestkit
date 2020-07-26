use std::env;
use std::ffi::{OsStr, OsString};

pub struct EnvironmentTestGuard {
    key: OsString,
}

pub fn set_env<V: AsRef<OsStr>>(key: OsString, value: V) -> EnvironmentTestGuard {
    env::set_var(&key, value);
    EnvironmentTestGuard { key }
}

impl Drop for EnvironmentTestGuard {
    fn drop(&mut self) {
        env::remove_var(&self.key)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use fake::faker::lorem::en::*;
    use fake::Fake;
    use serial_test::serial;

    fn gen_pair() -> (OsString, OsString) {
        (
            OsString::from(Word().fake::<String>()),
            OsString::from(Sentence(1..10).fake::<String>()),
        )
    }

    #[test]
    #[serial]
    fn set_env_ok() {
        let (key, value) = gen_pair();
        let _e = set_env(key.clone(), &value);
        assert_eq!(env::var_os(key).unwrap(), value)
    }

    #[test]
    #[serial]
    fn set_env_clean_up() {
        let (key, value) = gen_pair();
        {
            let _e = set_env(key.clone(), &value);
        }
        assert_eq!(env::var_os(key), None)
    }
}
