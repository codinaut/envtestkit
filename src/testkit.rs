use std::env;
use std::ffi::{OsStr, OsString};

#[must_use = "if unused environment will immediately be cleaned up"]
pub struct EnvironmentTestGuard {
    key: OsString,
    value: Option<OsString>,
}

pub fn set_env<V: AsRef<OsStr>>(key: OsString, value: V) -> EnvironmentTestGuard {
    let prev_value = env::var_os(&key);
    env::set_var(&key, value);

    EnvironmentTestGuard {
        key,
        value: prev_value,
    }
}

impl Drop for EnvironmentTestGuard {
    fn drop(&mut self) {
        match &self.value {
            Some(value) => env::set_var(&self.key, value),
            None => env::remove_var(&self.key),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use fake::faker::lorem::en::*;
    use fake::Fake;
    use serial_test::serial;

    fn gen_value() -> OsString {
        OsString::from(Sentence(1..10).fake::<String>())
    }

    fn gen_key_value() -> (OsString, OsString) {
        (OsString::from(Word().fake::<String>()), gen_value())
    }

    #[test]
    #[serial]
    fn set_env_ok() {
        let (key, value) = gen_key_value();
        let _e = set_env(key.clone(), &value);
        assert_eq!(env::var_os(key).unwrap(), value)
    }

    #[test]
    #[serial]
    fn set_env_clean_up() {
        let (key, value) = gen_key_value();
        {
            let _e = set_env(key.clone(), &value);
        }
        assert_eq!(env::var_os(key), None)
    }

    #[test]
    #[serial]
    fn set_env_restore() {
        let (key, value) = gen_key_value();
        env::set_var(&key, &value);
        {
            let _e = set_env(key.clone(), gen_value());
        }
        assert_eq!(env::var_os(key).unwrap(), value)
    }
}
