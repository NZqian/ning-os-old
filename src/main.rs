#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ning_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;
use bootloader::{BootInfo, entry_point};
use ning_os::allocator::init_heap;
use ning_os::hlt_loop;
use ning_os::memory::BootInfoFrameAllocator;
use ning_os::println;
use x86_64::structures::paging::Translate;
pub mod memory;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    ning_os::init();
    println!("hello world");
    x86_64::instructions::interrupts::int3();

    // trigger a stack overflow
    //stack_overflow();
    /*
    #[allow(unconditional_panic)]
    let a = 1 / 0;
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    }
    */
    /*
    use x86_64::VirtAddr;
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe {memory::init(phys_mem_offset)};
    let addresses = [
        0xb8000,
        0x201008,
        0x0100_0020_1a10,
        boot_info.physical_memory_offset,
    ];
    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }
    */
    use x86_64::{structures::paging::Page, VirtAddr};
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe{memory::init(phys_mem_offset)};
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    //mapper.translate_addr(page.start_address()).as_
    println!("map from {:?} to {:?}", page.start_address(), mapper.translate_addr(page.start_address()));
    println!("mut ptr {:?}", page_ptr);
    unsafe {
        let page_ptr_new = page_ptr.offset(0);
        println!("mut ptr new {:?}", page_ptr_new);
        page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e);
        page_ptr_new.write_volatile(0x_f021_f077_f065_f04e);
    }

    init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let x = Box::new(41);
    println!("heap value at {:p}", x);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));

    #[cfg(test)]
    test_main();
    hlt_loop();
}


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ning_os::test_panic_handler(info)
}