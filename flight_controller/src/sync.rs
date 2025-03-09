use core::{
    ops::{Deref, DerefMut}, 
    sync::atomic::{AtomicBool, Ordering},
    cell::{UnsafeCell, OnceCell}
};

pub struct Mutex<T> {
    inner: UnsafeCell<T>,
    is_locked: AtomicBool
}

impl <T> Mutex<T> {
    pub const fn new(value: T) -> Self {
        Self { 
            inner: UnsafeCell::new(value),
            is_locked: AtomicBool::new(false)
         }
    }

    pub fn lock(&self) -> Result<MutexGuard<T>, ()> {
        while let Ok(true) = self.is_locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed) {
        }
        Ok(MutexGuard { data: self })
    }
}

unsafe impl<T> Send for Mutex<T> {}
unsafe impl<T> Sync for Mutex<T> {}


pub struct MutexGuard<'guard, T> {
    data: &'guard Mutex<T>
}

impl<'guard, T> Deref for MutexGuard<'guard, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.data.inner.get()
        }
    }
}

impl <'guard, T> DerefMut for MutexGuard<'guard, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { 
            &mut *self.data.inner.get()
        }
    }
}

impl<'guard, T> Drop for MutexGuard<'guard, T> {
    fn drop(&mut self) {
        self.data.is_locked.store(false, Ordering::Release);
    }
}

pub struct OnceLock<T> {
    inner: OnceCell<T>,
    written: AtomicBool
}

impl <T> OnceLock<T> {
    pub const fn new() -> Self {
        Self {inner: OnceCell::new(), written: AtomicBool::new(false)}
    }

    // Can only fail, if it has already been written to
    pub fn set(&self, value: T) -> Option<()> {
        if let Ok(false) = self.written.compare_exchange(false, true, Ordering::Release, Ordering::Acquire) {
            self.inner.set(value).ok()
        } else {
            None
        }
    }

    pub fn get(&self) -> Option<&T> {
        self.inner.get()
    }
}

unsafe impl <T> Send for OnceLock<T> {}
unsafe impl <T> Sync for OnceLock<T> {}