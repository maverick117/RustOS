/* Interrupt service routine module */

/* Assembly function for setting up an interrupt gate for a service routine */
#[allow(dead_code)]
extern "C" {
    fn set_isr(    offset_1  : u16,
    selector  : u16,
    ist       : u8,
    type_attr : u8,
    offset_2  : u16,
    offset_3  : u32,
    zero      : u32);
}

/* Setup the interrupt gates */
pub fn init_isr(){

}

pub fn isr_set(int_num : u8, entry : IDTEntry){

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