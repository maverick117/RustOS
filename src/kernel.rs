/*
 *  Main source file for the Rust kernel
 *  Author: Jianzhong Liu
 *  All Rights Reserved
 */

#![feature(lang_items,asm,const_fn,core_intrinsics,use_extern_macros,naked_functions)]
#![no_std]


extern crate multiboot2;
extern crate rlibc;
extern crate spin;
#[macro_use]
extern crate bitflags;

mod vga_console;
mod isr;
mod mem;
mod process;

use vga_console::*;
use core::fmt::Write;
use isr::*;
use mem::*;

pub use isr::interrupt_handler;

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

 fn kernel_stack_overflow(i : i64){
     println!("Iteration {}",i);
     
     kernel_stack_overflow(i+1);
 }


#[no_mangle]
pub extern fn rust_start() -> !{
    //vga.lock().clear_screen();

    let finish_msg : &str = "done.\n";

    println!("Welcome to Rust Kernel."); // Print welcome banner


    println!("Multiboot address location: 0x{:x}", multiboot_loc);

    let boot_info = unsafe{multiboot2::load(multiboot_loc as usize)}; // Get info
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");
    println!("Memory areas:");

    for area in memory_map_tag.memory_areas() {
        println!("\tstart: 0x{:x}, length: 0x{:x}", area.base_addr, area.length);
    }

    let elf_sections_tag = boot_info.elf_sections_tag().expect("ELF sections tag required");
    println!("Kernel sections:");
    for section in elf_sections_tag.sections(){
        println!("\taddr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}", section.addr, section.size, section.flags);
    }

    let kernel_start = elf_sections_tag.sections().map(|section| section.addr).min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|section| section.addr + section.size).max().unwrap();

    let multiboot_start = multiboot_loc as usize;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    let mut frame_allocator = mem::AreaFrameAllocator::new(kernel_start as usize, kernel_end as usize, multiboot_start, multiboot_end, memory_map_tag.memory_areas());

    //println!("{:?}",frame_allocator.allocate_frame());
    for i in 0.. {
        if let None = frame_allocator.allocate_frame() {
            println!("allocated {} frames",i);
            break;
        }
    }
    loop{};
/*
    println!("Kernel start address: 0x{:x}", kernel_start);
    println!("Kernel  end  address: 0x{:x}", kernel_end);
    println!("Multiboot start address: 0x{:x}", multiboot_start);
    println!("Multiboot  end  address: 0x{:x}", multiboot_end);
    */
    // Initialize irqs

    init_isr();

    //kernel_stack_overflow(0);

    //unsafe{asm!("INT 0x20")};

    // Init page tables

    // Init scheduler

    // Setup timer interrupts, 10ms

    // enable interrupts


    // Transfer control to init program and transfer to user mode.
    //panic!();
    println!("Test");

    let mut num : isize = 0;
    loop{
        unsafe{*(0xdeadbeaf as *mut u64) = 50};
    };

    panic!();
    
    //unsafe{asm!("cli")};
    //unsafe{asm!("hlt")}; // Halt the machine
}


/* Make the Rust Compiler happy */
#[lang = "eh_personality"]
extern fn eh_personality() {}

/* Make the Rust Compiler happy */
#[lang = "panic_fmt"] #[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32)->!{
    unsafe{asm!("cli")}; // Clear interrupts
    println!("\n\nRUST KERNEL PANIC\tin {} at line {}:",file, line);
    println!("\t{}",fmt);
    println!("Kernel halted.");
    unsafe{asm!("hlt")}
    loop{}
}
