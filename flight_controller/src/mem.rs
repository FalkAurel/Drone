use core::{alloc::GlobalAlloc, mem::{self, MaybeUninit}, ptr::{null_mut, NonNull}};
use alloc::{alloc::{AllocError, Allocator}, slice};
use esp_println::println;
use crate::sync::{Mutex, MutexGuard};

const LOW_ADDR: usize  = 0x3F80_0000;
const HIGH_ADDR: usize = 0x3FBF_FFFF;

#[cfg(feature = "wifi")]
pub static ALLOCATOR: Mutex<BumpAllocator> = Mutex::new(BumpAllocator::new());

#[cfg(not(feature = "wifi"))]
#[global_allocator]
pub static ALLOCATOR: Mutex<BumpAllocator> = Mutex::new(BumpAllocator::new());

pub struct BumpAllocator {
    used_mem: usize,
    current: usize
}

impl BumpAllocator {
    pub const fn new() -> Self {
        Self { used_mem: 0, current: LOW_ADDR }
    }

    unsafe fn allocate(&mut self, size: usize, align: usize) -> Option<*mut u8> {
        let aligned_addr: usize = Self::align(self.current, align);
        //println!("Address: {} Size: {size} Align: {align} Aligned addr: {aligned_addr} Used Memory: {}", self.current, self.used_mem);
        if aligned_addr + size > HIGH_ADDR {
            None
        } else {
            let ptr: *mut u8 = aligned_addr as *mut u8;
            self.used_mem += size;
            
            self.current = aligned_addr + size;
            Some(ptr)
        }
    }

    unsafe fn clear(&mut self) {
        assert!(self.used_mem == 0, "Free is invalid");

        let start_ptr: *mut u8 = LOW_ADDR as *mut u8;
        
        start_ptr.write_bytes(0, self.current);
    }

    fn align(addr: usize, align: usize) -> usize {
        let remainder: usize = addr % align;

        if remainder == 0 {
            addr
        } else {
            addr - remainder + align
        }
    }
}

unsafe impl GlobalAlloc for Mutex<BumpAllocator> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let mut allocator: MutexGuard<BumpAllocator> = self.lock().unwrap();

        allocator.allocate(layout.size(), layout.align()).unwrap_or(null_mut())
    }

    unsafe fn dealloc(&self, _: *mut u8, layout: core::alloc::Layout) {
        let mut allocator: MutexGuard<BumpAllocator> = self.lock().unwrap();

        allocator.used_mem -= layout.size();

        if allocator.used_mem == 0 {
            allocator.clear();
        }
    }
}

#[cfg(feature = "wifi")]
unsafe impl Allocator for Mutex<BumpAllocator> {
    fn allocate(&self, layout: core::alloc::Layout) -> Result<core::ptr::NonNull<[u8]>, alloc::alloc::AllocError> {
        unsafe {
            match self.alloc(layout) {
                null_ptr if null_ptr.is_null() => Err(AllocError),
                ptr => {
                    assert!(layout.size() < isize::MAX as usize);        

                    let slice: &mut [u8] = slice::from_raw_parts_mut(ptr, layout.size());
                    assert!(ptr.is_aligned_to(mem::align_of_val(slice)));

                    Ok(NonNull::new(slice).unwrap())
                }
            }
        }
    }

    unsafe fn deallocate(&self, ptr: core::ptr::NonNull<u8>, layout: core::alloc::Layout) {
        //println!("Before");
        self.dealloc(ptr.as_ptr(), layout);
        //println!("After")
    }
}

pub fn mem_stats() {
    let allocator: MutexGuard<BumpAllocator> = ALLOCATOR.lock().unwrap();

    let total: usize = HIGH_ADDR - LOW_ADDR;
    let used_percentage: f32 = allocator.used_mem as f32 / total as f32;

    println!("{used_percentage}% of memory is used. {} of {total} bytes", allocator.used_mem)
}