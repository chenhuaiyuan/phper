use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;

use phper_sys::efree;
use phper_sys::emalloc;

pub struct EAllocator;

unsafe impl GlobalAlloc for EAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        emalloc!(layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        efree!(ptr as *mut c_void);
    }
}