use core::{
    ops::{Deref, DerefMut}, 
    sync::atomic::{AtomicBool, Ordering},
    cell::UnsafeCell
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