//! The main module and entrypoint
//!
//! The operating system and app also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality [`clear_bss()`]. (See its source code for
//! details.)
//!
//! We then call [`println!`] to display `Hello, world!`.

#![deny(missing_docs)]
// #![deny(warnings)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(asm_const)]
#![feature(naked_functions)]

use core::arch::{asm, global_asm};

#[macro_use]
mod console;
mod lang_items;
mod qemu_exit;
mod sbi;
use crate::qemu_exit::{QEMUExit, QEMU_EXIT_HANDLE};

global_asm!(include_str!("entry.asm"));

/// clear BSS segment
pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

const BOOTLOADER_STACK_SIZE: usize = 4096;
const CPUS: usize = 1;

#[link_section = ".bss.stack"]
static mut BOOTLOADER_STACK_SPACE: [[u8; BOOTLOADER_STACK_SIZE]; CPUS] =
    [[0; BOOTLOADER_STACK_SIZE]; CPUS];


// #[naked]
// #[no_mangle]
// #[link_section = ".text.entry"]
// unsafe extern "C" fn _start() {
//     asm!(
//         "la sp, {bootloader_stack}",
//         "li t0, {bootloader_stack_size}",
//         "csrr t1, mhartid",
//         "addi t1, t1, 1",
//         "mul t0, t0, t1",
//         "add sp, sp, t0",
//         "j {rust_start}",
//         bootloader_stack = sym BOOTLOADER_STACK_SPACE,
//         bootloader_stack_size = const BOOTLOADER_STACK_SIZE,
//         rust_start = sym rust_main,
//         options(noreturn),
//     );
// }

/// the rust entry-point of os
#[no_mangle]
pub fn rust_main() -> ! {
    // extern "C" {
    //     fn stext(); // begin addr of text segment
    //     fn etext(); // end addr of text segment
    //     fn srodata(); // start addr of Read-Only data segment
    //     fn erodata(); // end addr of Read-Only data ssegment
    //     fn sdata(); // start addr of data segment
    //     fn edata(); // end addr of data segment
    //     fn sbss(); // start addr of BSS segment
    //     fn ebss(); // end addr of BSS segment
    //     fn boot_stack(); // stack bottom
    //     fn boot_stack_top(); // stack top
    // }
    clear_bss();
    println!("Hello, world!");
    // println!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    //  println!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    // println!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    // println!(
    //      "boot_stack [{:#x}, {:#x})",
    //      boot_stack as usize, boot_stack_top as usize
    //  );
    // println!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    
    QEMU_EXIT_HANDLE.exit_success();  //need successful qemu exit
    //QEMU_EXIT_HANDLE.exit_failure(); //need failed qemu exit
    // panic!("Shutdown machine!");
}
