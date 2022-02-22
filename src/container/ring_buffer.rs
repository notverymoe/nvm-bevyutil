
const CACGE_LINE_BYTES: usize = 64;
const CACHE_BYTES: usize = CACGE_LINE_BYTES - 15;

#[repr(align(64))]
pub struct RingBuffer<T: Copy> {
    /* .2 */ head:     u16,
    /* .2 */ tail:     u16,
    /* .2 */ capacity: u16,
    /* .8 */ data: *mut u8,
    /* -1 */ cache_head: u8,
    /*----*/
    /* 15 */

    cache: [u8; CACHE_BYTES],

    __: core::marker::PhantomData<*mut T>,
}

impl<T: Copy> Drop for RingBuffer<T> {

    fn drop(&mut self) {
        core::mem::drop( unsafe{ Box::from_raw(core::slice::from_raw_parts_mut(self.data, (self.capacity + 1) as usize * core::mem::size_of::<T>())) } );
    }

}


impl<T: Copy> RingBuffer<T> {

    const CACHE_LINE_LEN: usize = (CACHE_BYTES / core::mem::size_of::<T>());

    pub fn new(capacity: u16) -> Self {
        debug_assert!(core::mem::align_of::<T>() <= CACGE_LINE_BYTES);
        let layout = std::alloc::Layout::from_size_align((capacity + 1) as usize * core::mem::size_of::<T>(), CACGE_LINE_BYTES).unwrap();

        Self {
            head: 0,
            tail: 0,
            capacity,
            cache_head: 0,
            data: unsafe{ std::alloc::alloc(layout) },
            cache: [0; CACHE_BYTES],
            __: Default::default(),
        }
    }

    // // LEN // //

    pub fn len(&self) -> usize {
        (if self.tail < self.head { self.tail + self.capacity } else { self.tail } - self.head) as usize
    }

    pub fn is_empty(&self) -> bool {
        self.tail == self.head
    }

    pub fn is_full(&self) -> bool {
        self.tail + 1 == self.head || self.tail == self.capacity && self.head == 0
    }

    // // PUSH // //

    pub fn push(&mut self, v: T) -> bool {
        if self.is_full() { return false; }
        unsafe { self.push_unchecked(v); }
        true
    }

    pub unsafe fn push_unchecked(&mut self, v: T) {
        core::ptr::copy_nonoverlapping(&v, match self.get_forward_length() {
            Some(forward_len)  if (forward_len as usize) < Self::CACHE_LINE_LEN => (self.cache.as_mut_ptr() as *mut T).offset(forward_len as isize),
            _ => (self.cache.as_mut_ptr() as *mut T).offset(self.tail as isize)
        }, 1);
        self.advance_tail_unchecked();
    }

    // // HEAD // //

    pub fn advance_head(&mut self) -> bool {
        if self.head != self.tail {
            unsafe{ self.advance_head_unchecked(); }
            true
        } else {
            false
        }
    }

    pub unsafe fn advance_head_unchecked(&mut self) {
        self.head       = if self.head == self.capacity { 0 } else { self.head + 1 };
        self.cache_head += 1;

        if self.cache_head as usize >= Self::CACHE_LINE_LEN {
            self.cache_head = 0;
            core::ptr::copy_nonoverlapping(
                self.data.offset(self.head as isize * core::mem::size_of::<T>() as isize), 
                self.cache.as_mut_ptr(), 
                CACHE_BYTES
            )
        }
    }

    // // POP // //

    pub fn pop(&mut self) -> Option<T> {
        match self.head == self.tail {
            true  => None,
            false => Some(unsafe{ self.pop_unchecked() }),
        }
    }

    pub unsafe fn pop_unchecked(&mut self) -> T {
        let value = *self.peek_unchecked();
        self.advance_head_unchecked();
        value
    }

    // // PEEK // //

    pub fn peek(&self) -> Option<&T> {
        match self.head == self.tail {
            true  => None,
            false => Some(unsafe{ self.peek_unchecked() }),
        }
    }

    pub unsafe fn peek_unchecked(&self) -> &T {
        & *(self.cache.as_ptr() as *const T).offset(self.cache_head as isize)
    }

    pub unsafe fn peek_unchecked_mut(&mut self) -> &mut T {
        &mut *(self.cache.as_mut_ptr() as *mut T).offset(self.cache_head as isize)
    }


    // // INTERNAL // //

    unsafe fn advance_tail_unchecked(&mut self) {
        self.tail = if self.tail == self.capacity { 0 } else { self.tail+ 1 };
    }

    fn get_forward_length(&self) -> Option<u16> {
        if self.tail > self.head { Some(self.tail - self.head) } else { None }
    }

}