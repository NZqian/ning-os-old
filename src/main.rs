#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ning_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use ning_os::hlt_loop;
use ning_os::memory::BootInfoFrameAllocator;
use ning_os::println;
use ning_os::print;
use x86_64::structures::paging::PageTable;
use x86_64::structures::paging::Translate;
pub mod memory;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    ning_os::init();
    println!("hello world");
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
    unsafe {
        page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e);
    }
    #[cfg(test)]
    test_main();
    //loop {}
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