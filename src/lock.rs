use parking_lot::lock_api::{RwLockReadGuard, RwLockWriteGuard};
use parking_lot::RawRwLock;
use parking_lot::RwLock;

lazy_static! {
    static ref LOCK: RwLock<()> = RwLock::new(());
}

pub fn lock_read<'a>() -> RwLockReadGuard<'a, RawRwLock, ()> {
    LOCK.read()
}

pub fn lock_test<'a>() -> RwLockWriteGuard<'a, RawRwLock, ()> {
    LOCK.write()
}
