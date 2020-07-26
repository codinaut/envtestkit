use parking_lot::lock_api::{RwLockReadGuard, RwLockWriteGuard};
use parking_lot::RawRwLock;
use parking_lot::RwLock;

lazy_static! {
    static ref LOCK: RwLock<()> = RwLock::new(());
}

/// Ensure no environment-modifying tests are running at the moment.
///
/// This method should be called before running environment-sensitive tests.
///
/// # Examples
///
/// ```
/// use envtestkit::lock::lock_read;
///
/// let _lock = lock_read();
/// ```
pub fn lock_read<'a>() -> RwLockReadGuard<'a, RawRwLock, ()> {
    LOCK.read()
}

/// Serialize environment-modifying tests.
///
/// This method should be called before running environment-modifying tests.
///
/// # Examples
///
/// ```
/// use envtestkit::lock::lock_test;
///
/// let _lock = lock_test();
/// ```
pub fn lock_test<'a>() -> RwLockWriteGuard<'a, RawRwLock, ()> {
    LOCK.write()
}
