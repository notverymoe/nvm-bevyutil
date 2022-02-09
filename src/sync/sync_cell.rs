/*========================================================================*\
** NotVeryMoe BevyUtil | Copyright 2021 NotVeryMoe (projects@notvery.moe) **
\*========================================================================*/

use std::{sync::atomic::{Ordering, AtomicU8}, cell::UnsafeCell, ops::{Deref, DerefMut}};

const BORROW_MUT_FLAG: u8 = 1 << (u8::BITS - 1);

/**
 * Implements a userspace rwlock-style mutex where writer
 * contention is an error. Implemented with an interface
 * similar to a RefCell. Has a limit of 2^7 readers, and
 * will panic when exceeded.
 **/ 
pub struct SyncCell<T: ?Sized>(AtomicU8, UnsafeCell<T>);

unsafe impl<T: ?Sized + Send> Send for SyncCell<T> {}
unsafe impl<T: ?Sized + Sync> Sync for SyncCell<T> {}

impl<T> SyncCell<T> {
    #[inline] pub fn new(v: T) -> Self {
        Self(0.into(), UnsafeCell::new(v))
    }

    #[inline] pub fn into_inner(self) -> T {
        assert!(self.0.load(Ordering::Acquire) == 0); // Force sync, check refcount
        self.1.into_inner()
    }
}

impl<T: ?Sized + Default> Default for SyncCell<T> {
    #[inline] fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}

impl<T: ?Sized> SyncCell<T> {
    #[inline] pub fn borrow(&self) -> SyncRef<'_, T> {
        if self.0.fetch_add(1, Ordering::AcqRel) < BORROW_MUT_FLAG {
            SyncRef(self)
        } else {
            unreachable!()
        }
    }

    #[inline] pub fn borrow_mut(&self) -> SyncMutRef<'_, T> {
        if self.0.fetch_or(BORROW_MUT_FLAG, Ordering::AcqRel) == 0 {
            SyncMutRef(self)
        } else {
            unreachable!()
        }
    }

    #[inline] pub fn get_mut(&mut self) -> &mut T {
        assert!(self.0.load(Ordering::Acquire) == 0); // Force sync, check refcount
        self.1.get_mut()
    }
}

impl<T: ?Sized> SyncCell<T> {
    #[inline] fn release(&self) {
        self.0.fetch_sub(1, Ordering::AcqRel);
    }

    #[inline] fn release_mut(&self) {
        self.0.store(0, Ordering::Release);
    }
}

impl<T> From<T> for SyncCell<T> {
    #[inline] fn from(v: T) -> Self {
        Self::new(v)
    }
}

pub struct SyncRef<'a, T: ?Sized>(&'a SyncCell<T>);

impl<'a, T: ?Sized> Deref for SyncRef<'a, T> {
    type Target = T;
    #[inline] fn deref(&self) -> &Self::Target {
        unsafe{ &*self.0.1.get() }
    }
}

impl<'a, T: ?Sized> Drop for SyncRef<'a, T> {
    #[inline] fn drop(&mut self) {
        self.0.release();
    }
}


pub struct SyncMutRef<'a, T: ?Sized>(&'a SyncCell<T>);

impl<'a, T: ?Sized> Deref for SyncMutRef<'a, T> {
    type Target = T;
    #[inline] fn deref(&self) -> &Self::Target {
        unsafe{ &*self.0.1.get() }
    }
}

impl<'a, T: ?Sized> DerefMut for SyncMutRef<'a, T> {
    #[inline] fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe{ &mut *self.0.1.get() }
    }
}

impl<'a, T: ?Sized> Drop for SyncMutRef<'a, T> {
    fn drop(&mut self) {
        self.0.release_mut();
    }
}