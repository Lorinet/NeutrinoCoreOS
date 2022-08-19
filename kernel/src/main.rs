#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(neutrino_os::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use neutrino_os::*;
use core::panic::PanicInfo;

#[cfg(target_arch = "x86_64")]
use {
    dev::hal::*,
    dev::framebuffer::*,
    dev::char::*,
    bootloader::{BootInfo, entry_point}
};
#[cfg(target_arch = "x86_64")]
entry_point!(kernel_main);
#[cfg(target_arch = "x86_64")]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    //loop {}
    unsafe { 
        dev::hal::mem::PHYSICAL_MEMORY_OFFSET = boot_info.physical_memory_offset.into_option().unwrap();
        dev::hal::mem::BOOT_MEMORY_MAP = Some(&boot_info.memory_regions);
        let free_mem = boot_info.memory_regions.iter().map(|reg| reg.end - reg.start);
        let free_mem: u64 = free_mem.sum();
        dev::hal::mem::FREE_MEMORY = free_mem as usize;
        let bifb = boot_info.framebuffer.as_mut().unwrap();
        let bifbi = bifb.info();
        kernel_console::FRAMEBUFFER = Some(VesaVbeFramebuffer::new(bifb.buffer_mut(), bifbi.horizontal_resolution, bifbi.vertical_resolution,
    match bifbi.pixel_format {
            bootloader::boot_info::PixelFormat::RGB => dev::framebuffer::PixelFormat::RGB,
            bootloader::boot_info::PixelFormat::BGR => dev::framebuffer::PixelFormat::BGR,
            bootloader::boot_info::PixelFormat::U8 => dev::framebuffer::PixelFormat::Monochrome,
            _ => dev::framebuffer::PixelFormat::RGB,
        }, bifbi.bytes_per_pixel, bifbi.stride));
        kernel_console::KERNEL_CONSOLE = Some(FramebufferConsole::new(kernel_console::FRAMEBUFFER.as_mut().unwrap()));
    }
    #[cfg(test)]
    test_main();
    kernel::run_kernel()
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        //kernel_console::KERNEL_CONSOLE.disable_cursor();
    }
    neutrino_os::panic::panic(info)
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    neutrino_os::panic::test_panic(info)
}