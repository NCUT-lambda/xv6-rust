#![no_std]
#![feature(default_alloc_error_handler)]
#![feature(c_size_t)]

extern crate alloc;

use alloc::ffi::CString;
use alloc::string::ToString;
use core::alloc::GlobalAlloc;
use core::panic::PanicInfo;
use core::ptr;

mod xv6_ulib {
    extern "C" {
        pub fn malloc(_: core::ffi::c_uint) -> *mut core::ffi::c_void;
        pub fn free(_: *mut core::ffi::c_void) -> core::ffi::c_void;
    }
}

struct Xv6GlobalAllocator {}

unsafe impl GlobalAlloc for Xv6GlobalAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        todo!()
    }
}

#[global_allocator]
static XV6_GLOBAL_ALLOCATOR: Xv6GlobalAllocator = Xv6GlobalAllocator {};

mod xv6_printf {
    extern "C" {
        pub fn panic(info: *const core::ffi::c_char) -> !;
    }
}

#[panic_handler]
fn rust_panic_handler(info: &PanicInfo) -> ! {
    let info = info.to_string();
    let info = CString::new(info)
        .map(|info| info.as_ptr())
        .unwrap_or_else(|_| core::ptr::null());
    unsafe { xv6_printf::panic(info) }
}
