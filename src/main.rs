#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ning_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ning_os::hlt_loop;
use ning_os::println;
use ning_os::print;

#[no_mangle]
pub extern "C" fn _start() -> ! {
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