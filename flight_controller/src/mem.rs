const LOW_ADDRESS: usize = 0x3F80_0000;
const HIGH_ADDRESS: usize = 0x3FBF_FFFF;

#[global_allocator]
pub static ALLOCATOR: Mutex<LinkedListAllocator> = Mutex::new(LinkedListAllocator::new());

use core::{alloc::{GlobalAlloc, Layout}, ptr};

use address::Address;
use alignment::*;
use esp_println::println;
use memory::{Node, Region};

use crate::sync::{Mutex, MutexGuard};

mod address {
    use core::{ops::{Add, Sub}, usize};

    #[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
    pub struct Address(usize);

    impl Address {
        pub const fn new(value: usize) -> Self {
            Self(value)
        }
    }

    impl Add<usize> for Address {
        type Output = Address;

        fn add(self, rhs: usize) -> Self::Output {
            Address(self.0 + rhs)
        }
    }

    impl Add<Address> for usize {
        type Output = Address;

        fn add(self, rhs: Address) -> Self::Output {
            Address(self + rhs.0)
        }
    }


    impl Add<Address> for Address {
        type Output = Address;

        fn add(self, rhs: Address) -> Self::Output {
            Address(self.0 + rhs.0)
        }
    }

    impl Sub<usize> for Address {
        type Output = Address;

        fn sub(self, rhs: usize) -> Self::Output {
            Address(self.0 - rhs)
        }
    }

    impl Sub<Address> for usize {
        type Output = Address;

        fn sub(self, rhs: Address) -> Self::Output {
            Address(self - rhs.0)
        }
    }

    impl Sub<Address> for Address {
        type Output = Address;

        fn sub(self, rhs: Address) -> Self::Output {
            Address(self.0 - rhs.0)
        }
    }

    impl From<usize> for Address {
        fn from(value: usize) -> Self {
            Address(value)
        }
    }

    impl From<Address> for usize {
        fn from(value: Address) -> Self {
            value.0
        }
    }
}


mod alignment {
    use core::alloc::Layout;
    use super::{address::Address, memory::Node};

    /// Returns an upwards aligned starting address to the memory block with a given size and alignment
    pub fn align_up(addr: Address, size: usize, align: usize) -> Address {
        let remainder: usize = size % align; // 0 < remainder < size

        if remainder == 0 { // is already aligned
            addr
        } else {
            addr + remainder
        }
    }

    pub fn align_layout(layout: Layout) -> (usize, usize) {
        let aligned_layout: Layout = layout.align_to(align_of::<Node>())
        .expect("Alignment failed")
        .pad_to_align();

        (aligned_layout.size().max(size_of::<Node>()), layout.align())
    }
}

mod memory {
    use super::address::Address;

    pub struct Node {
        pub(crate) size: usize,
        pub(crate) next: Option<&'static mut Node>
    }

    impl Node {
        pub const fn new(size: usize) -> Self {
            Self { size, next: None }
        }

        pub fn start_address(&self) -> Address {
            (self as *const Self as usize).into()
        }

        pub fn end_address(&self) -> Address {
            self.start_address() + self.size
        }
    }

    pub struct Region<'region> {
        pub(crate) region: &'region Node,
        pub(crate) alloc_start_addr: Address
    }
}


pub struct LinkedListAllocator {
    head: Node,
    used_mem: usize
}

impl LinkedListAllocator {
    pub const fn new() -> Self {
        Self { head: Node::new(0), used_mem: 0 }
    }

    unsafe fn add_free_region(&mut self, addr: Address, size: usize) {
        self.push_node(addr, size);
    }

    /// Creates a new node and pushes it to the beginning of the free list 
    unsafe fn push_node(&mut self, addr: Address, size: usize) {
        let mut new_head: Node = Node::new(size);
        new_head.next = self.head.next.take(); // Move old head node

        let new_head_ptr: *mut Node = Into::<usize>::into(addr) as *mut Node;
        ptr::write(new_head_ptr, new_head); // Write new_head to memory
        self.head.next = Some(&mut *new_head_ptr) // Push new_head to the front
    }


    pub fn find_region(&mut self, size: usize, align: usize) -> Option<Region> {
        let mut current: &mut Node = &mut self.head;

        while let Some(region) = &mut current.next {
            if let Some(alloc_start) = Self::verify_region(&region, size, align) {
                let next: Option<&mut Node> = region.next.take();
                let ret: Option<Region> = Some(Region { region: current.next.take().unwrap(), alloc_start_addr: alloc_start });
                current.next = next;

                return ret
            } else {
                current = current.next.as_mut().unwrap();
            }
        }

        None
    }
    
    fn verify_region(region: &Node, size: usize, align: usize) -> Option<Address> {
        let allocation_start: Address = align_up(region.start_address(), size, align);
        let allocation_end:   Address = allocation_start + size;

        if region.end_address() < allocation_end { // Aligned data will exceed the region; region is too small
            return None
        }

        // Previous guard clause gurantees that region_end >= alloc_end
        let excess: usize = (region.end_address() - allocation_end).into();

        if excess > 0 && excess < size_of::<Node>() {
            return None
        }
        
        Some(allocation_start)
    }
}

unsafe impl GlobalAlloc for Mutex<LinkedListAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let (size, align) = align_layout(layout);
        let mut allocator: MutexGuard<LinkedListAllocator> = self.lock().unwrap();

        if let Some(Region { region, alloc_start_addr }) = allocator.find_region(size, align) {
            assert!(region.next.is_none(), "Sanity check failed");

            let alloc_end: Address = alloc_start_addr + size;
            let excess: usize = (region.end_address() - alloc_end).into();

            if excess > 0 {
                allocator.add_free_region(alloc_end, excess) 
            }

            allocator.used_mem += size;
            Into::<usize>::into(alloc_start_addr) as *mut u8
        } else {
            ptr::null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let (size, _) = align_layout(layout);
        let mut allocator: MutexGuard<LinkedListAllocator> = self.lock().unwrap();
        allocator.add_free_region((ptr as usize).into(), size);

        allocator.used_mem -= size;
    }
}


pub fn init_heap() {
    unsafe {
        ALLOCATOR.lock().unwrap().add_free_region(LOW_ADDRESS.into(), HIGH_ADDRESS - LOW_ADDRESS);
    }
}

pub fn get_mem_stats() {
    let unused: usize = HIGH_ADDRESS - LOW_ADDRESS - ALLOCATOR.lock().unwrap().used_mem;

    println!("Heap has {unused} bytes left")
}
