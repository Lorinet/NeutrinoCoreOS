use crate::*;
use console::ConsoleColor;
use crate::dev::Write;
use crate::userspace::*;
use dev::input::keyboard;
use dev::StaticDevice;
use sysinfo;
use dev;
use dev::hal::*;
use async_task::*;
use crate::task::scheduler::*;
use enum_iterator::all;
use alloc::vec::Vec;

pub fn run_kernel() -> ! {
    
    kernel_console::set_color(ConsoleColor::LightGray,  ConsoleColor::Black);
    kernel_console::clear_screen();
    init_system();
    loop {
        cpu::halt();
    }
}

fn init_system() {
    early_print!("Linfinity Technologies Neutrino Core OS [Version {}]\n", sysinfo::NEUTRINO_VERSION);
    dev::hal::init();
    early_print!("[{} MB memory available]\n", unsafe { mem::FREE_MEMORY } / 1048576);
    kernel_executor::init();
    dev::input::PS2KeyboardPIC8259::set_input_handler(test_input_keyboard);
    dev::input::PS2KeyboardPIC8259::init_device().unwrap();
    //Scheduler::exec(userspace_app_1);
    Scheduler::kexec(kernel_executor::run);
    cpu::enable_scheduler();
    kernel_executor::run();
}

fn test_input_keyboard(key: keyboard::Key) {
    if let keyboard::Key::Unicode(ch) = key {
        print!("{}", ch);
    }
}