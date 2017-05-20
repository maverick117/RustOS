/*
 *  Main source file for the Rust kernel
 *  Author: Jianzhong Liu
 *  All Rights Reserved
 */

#![feature(lang_items,asm,const_fn)]
#![no_std]


extern crate multiboot2;
extern crate rlibc;
extern crate spin;

mod vga_console;

use vga_console::*;
use core::fmt::Write;


#[allow(dead_code)]
extern "C" {
    fn set_isr(interrupt_num:u64, function_address:u64);
}


extern {
    static multiboot_loc: u32;
}


/*
 *   RustOS Kernel Start function
 *   Interrupts: Disabled
 *   Mode: Long Mode on x86
 *   IDT: Set, awaiting isr setup
 *   GDT: Set to flat memory
 *   Paging: Identity paging set
 */


#[no_mangle]
pub extern fn rust_start(){
    vga.lock().clear_screen();

    let finish_msg : &str = "done.\n";

    println!("Welcome to RustOS.");

    //write!(vga,"{}",1.0/3.0);
    let boot_info = unsafe{multiboot2::load(multiboot_loc as usize)}; // Get info
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");
    println!("Memory areas:");

    for area in memory_map_tag.memory_areas() {
        println!("\tstart: 0x{:x}, length: 0x{:x}", area.base_addr, area.length);
    }
    
    // Initialize irqs

    // Init page tables

    // Init scheduler

    // Setup timer interrupts, 10ms

    // enable interrupts

    
    // Transfer control to init program and transfer to user mode.

    
    unsafe{asm!("hlt")}; // Halt the machine
}


/* Make the Rust Compiler happy */
#[lang = "eh_personality"]
extern fn eh_personality() {}

/* Make the Rust Compiler happy */
#[lang = "panic_fmt"] #[no_mangle]
pub extern fn panic_fmt()->!{loop{}}
