#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ning_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use ning_os::hlt_loop;
use ning_os::println;
use ning_os::print;
use x86_64::structures::paging::PageTable;
use x86_64::structures::paging::Translate;
pub mod memory;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    /*
    for i in 0..2000 {
        println!("this is the {} row", i);
    }
    */
    //panic!("fuck!");
    ning_os::init();
    println!("hello world");
    //x86_64::instructions::interrupts::int3();
    println!("it worked!");
    /*
    fn stack_overflow() {
        stack_overflow()
    }
    stack_overflow();
    */
    /*
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    }
    */
    /* 
    loop{
        for i in 0..1000 {
        }
            print!("-");
    }
    */
    /* 
    use x86_64::VirtAddr;
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe {actived_level_4_table(phys_mem_offset)};
    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
            let phys = entry.frame().unwrap().start_address();
            let virt = phys.as_u64() + boot_info.physical_memory_offset;
            let ptr = VirtAddr::new(virt).as_mut_ptr();
            let l3_table: &PageTable = unsafe {&mut *ptr};
            for (i, entry) in l3_table.iter().enumerate() {
                if !entry.is_unused() {
                    println!("  L3 Entry {}: {:?}", i, entry);
                }
            }
        }
    }
    */
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