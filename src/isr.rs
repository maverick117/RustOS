/* Interrupt service routine module */



/* Assembly function for setting up an interrupt gate for a service routine */
#[allow(dead_code)]
extern {
    fn set_isr_gate(num :usize, addr:usize);
    fn set_default_isr();
}


/* Setup the interrupt gates */
#[inline]
pub fn init_isr(){
    //println!("GDT address: 0x{:x}, IDT address: 0x{:x}", gdt64, idt64);
    
    unsafe{set_default_isr()};
    unsafe{asm!("sti")};
    unsafe{asm!("nop")};
}

#[repr(C)]
#[derive(Copy,Clone,Debug)]
pub struct IDTEntry {
    offset_1  : u16,
    selector  : u16,
    ist       : u8,
    type_attr : u8,
    offset_2  : u16,
    offset_3  : u32,
    zero      : u32,
}

#[no_mangle]
#[allow(dead_code)]
pub extern "C" fn interrupt_handler(num: usize, errno: usize){
    println!("Interrupt {} called.", num);
}

pub struct trap_registers {
    
}