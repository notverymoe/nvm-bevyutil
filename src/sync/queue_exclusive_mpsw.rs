/*========================================================================*\
** NotVeryMoe BevyUtil | Copyright 2021 NotVeryMoe (projects@notvery.moe) **
\*========================================================================*/
use std::{sync::{Mutex, atomic::{Ordering, AtomicUsize}, MutexGuard, PoisonError}, cell::UnsafeCell};

/// This is a queue with no ordering guarentees, that offers low-contention multi-writer
/// access and requires exclusive access to the structure to perform read operations. This
/// is primarily useful for storing return values from concurrent batch tasks for later
/// reading after fully proccessed - ie. in Bevy_ECS.
/// 
/// Generic Params:
/// - [`T`] The datatype to store - consider a timestamp for manual sorting if order is desired
/// - [`N`] The number of writer slots to allocate. Contention will occur if N+1 writers are borrowed.
///         However, too many slots may slow iteration down with empty chained iterators. Heuristics.
pub struct QueueExclusiveMPSC<T, const N: usize> {
    index: AtomicUsize,
    data: [(Mutex<()>, UnsafeCell<Vec<T>>); N],
}

unsafe impl<T: Send, const N: usize> Send for QueueExclusiveMPSC<T, N> {}
unsafe impl<T: Sync, const N: usize> Sync for QueueExclusiveMPSC<T, N> {}

/// Allows us to erase `N` when exposing a reference to the writer, via `impl QueueExclusiveMPSCWriterSource<T>`.
pub trait QueueExclusiveMPSCWriterSource<T> { 
    fn borrow_writer(&self) -> QueueExclusiveMPSCWriter<'_, T>;
}

impl<T, const N: usize> QueueExclusiveMPSCWriterSource<T> for QueueExclusiveMPSC<T, N> {
    fn borrow_writer(&self) -> QueueExclusiveMPSCWriter<'_, T> {
        self.borrow_writer() // Calls self implementation bellow, not inf recursion
    }
}

impl<T, const N: usize> Default for QueueExclusiveMPSC<T, N> {
    fn default() -> Self {
        Self {
            index: Default::default(),
            data: [(); N].map(|_| Default::default()),
        }
    }
}

impl<T, const N: usize> QueueExclusiveMPSC<T, N> {
    /// Borrows a writer from the structure.
    /// Multiple calls in the same thread will return new writers each time, and may cause a deadlock
    /// if previous borrows are not returned.
    pub fn borrow_writer(&self) -> QueueExclusiveMPSCWriter<'_, T> {
        let (mutex, data) = &self.data[self.index.fetch_add(1, Ordering::Relaxed) % N];
        let lock = mutex.lock().unwrap_or_else(PoisonError::into_inner); // `Vec` should be well-defined on `panic!` in `push`
        QueueExclusiveMPSCWriter( unsafe{ &mut *data.get() }, lock) // UNSAFE We have the lock, so accessing UnsafeCell as mutable is safe
    }

    /// Iterates over the slots' contents immutably
    pub fn iter(&mut self) -> impl Iterator<Item = &T> {
        self.data.iter_mut().map(|v| &*v.1.get_mut()).flatten()
    }

    /// Iterates over the slots' contents mutably
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut().map(|v| v.1.get_mut()).flatten()
    }

    /// Clears teh queue of all data
    pub fn clear(&mut self) {
        self.index.store(0, Ordering::Relaxed); // Reset to 0, should reduce reallocations on insert
        for (_, queue) in &mut self.data {
            //if lock.is_poisoned() { *lock = Default::default() // We don't currently "unpoison", since we ignore it anyway, needs profiling
            queue.get_mut().clear();
        }
    }

    /// Takes ownership of the contents from all slots. Probbably slower than iteration...
    pub fn take(&mut self) -> [Vec<T>; N] {
        // Stabalize `each_mut` please...
        let mut idx = 0;
        [(); N].map(|_| {
            let v = self.data[idx].1.get_mut();
            idx += 1;
            std::mem::replace(v, Vec::with_capacity(v.capacity())) // Replace with matching capacity to reduce reallocations
        })
    }

    /// NOTE iterates the structure, slow.
    pub fn len(&mut self) -> usize {
        self.data.iter_mut().map(|v| v.1.get_mut().len()).sum()
    }

    /// NOTE iterates the structure, slow.
    pub fn is_empty(&mut self) -> bool {
        self.len() == 0
    }
}

// Functions only provided for diagnostics/ahead-of-time-optimization
#[cfg(debug_assertions)]
impl<T, const N: usize> QueueExclusiveMPSC<T, N> {
    /// Returns the number of writer aqcuisions, useful for magic-number tuning of slot counts.
    /// This number will be wrong if more than usize::MAX are aqcuired before calling `Self::clear`.
    pub fn acquisitions(&self) -> usize {
        self.index.load(Ordering::Relaxed)
    }

    /// Returns the writers with data, useful for magic-number tuning of slot counts.
    /// May provide misleading information if borrowed writers aren't used.
    pub fn used_writers(&mut self) -> usize {
        self.data.iter_mut().fold(0, |p, v| p + if v.1.get_mut().is_empty() { 0 } else { 1 })
    }

    /// Returns maximium writer index+1 with data, useful for magic-number tuning of slot counts. Only useful with
    /// manual index resets, otherwise effectively it will return `min(N, self.acquisitions())`. Returns zero if no
    /// slots contain data. May provide misleading information if borrowed writers aren't used.
    pub fn max_writers(&mut self) -> usize {
        match self.data.iter().enumerate().filter(|(_, v)| !unsafe{&*v.1.get()}.is_empty()).last() {
            Some(v) => v.0+1,
            None    => 0,
        }
    }
}

/// The borrowed writer to append data to the queue
pub struct QueueExclusiveMPSCWriter<'a, T>(&'a mut Vec<T>, MutexGuard<'a, ()>);

impl<'a, T> QueueExclusiveMPSCWriter<'a, T> {
    /// Inserts `v` into the queue.
    pub fn insert(&mut self, v: T) {
        self.0.push(v)
    }
}