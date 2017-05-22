/* Interrupt service routine module */

macro_rules! handler {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe{
                asm!("mov rdi, rsp
                      sub rsp, 8
                      call $0"
                      :: "i" ($name as extern "C" fn (&ExceptionStackFrame)) : "rdi" : "intel", "volatile");
                asm!("add rsp, 8
                      iretq"
                      ::::"intel","volatile");
                ::core::intrinsics::unreachable();
            }
        }
        wrapper as usize
    }}
}

macro_rules! handler_with_error_code {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe{
                asm!("pop rsi
                      mov rdi, rsp
                      sub rsp, 8
                      call $0"
                      :: "i" ($name as extern "C" fn (&ExceptionStackFrame, u64) -> !) : "rdi", "rsi" : "intel");
                ::core::intrinsics::unreachable();
            }
        }
        wrapper as usize
    }}
}


/* Assembly function for setting up an interrupt gate for a service routine */
#[allow(dead_code)]
extern {
    fn set_isr_gate(num :usize, addr:usize);
    fn set_default_isr();
    fn invoke_breakpoint();
}


/* Setup the interrupt gates */
pub fn init_isr(){
    //println!("GDT address: 0x{:x}, IDT address: 0x{:x}", gdt64, idt64);
    
    unsafe{set_default_isr()};
    unsafe{set_isr_gate(0,handler!(div_by_zero_handler))};

    unsafe{set_isr_gate(3,handler!(breakpoint_handler))};

    unsafe{set_isr_gate(6,handler!(invalid_opcode_handler))};

    unsafe{set_isr_gate(14,handler_with_error_code!(page_fault_handler))};  
    
    loop{
        unsafe{invoke_breakpoint()};
    }
    //unsafe{asm!("int $3"::::"intel")};

    unsafe{asm!("ud2")};
    unsafe{*(0xdeadbeaf as *mut u64) = 42};
    unsafe{asm!("mov dx, 0; div dx" ::: "ax","dx" : "volatile" , "intel")};
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
pub extern "C" fn interrupt_handler(stack_frame : & ExceptionStackFrame){
    println!("Interrupt.\n{:#?}", unsafe{&*stack_frame});
    
    loop{};
}

#[derive(Debug)]
#[repr(C)]
pub struct ExceptionStackFrame {
    //idt_num : u64,
    instruction_pointer : u64,
    code_segment : u64,
    cpu_flags : u64,
    stack_pointer : u64,
    stack_segment : u64,
}


/* Interrupt 0, div by zero */
#[no_mangle]
extern "C" fn div_by_zero_handler(stack_frame : &ExceptionStackFrame) {
    println!("\nPROCESSOR EXCEPTION: DIV_BY_ZERO.\n{:#?}",unsafe{&*stack_frame});
    loop{};
}

/* Interrupt 3, breakpoint */
extern "C" fn breakpoint_handler(stack_frame : &ExceptionStackFrame) {
    let stack_frame = unsafe{&*stack_frame};
    println!("\nPROCESSOR EXCEPTION: BREAKPOINT at {:#x}\n{:#?}",
        stack_frame.instruction_pointer, stack_frame
    );
    
}

extern "C" fn invalid_opcode_handler(stack_frame : &ExceptionStackFrame) {
    println!("\nPROCESSOR EXCEPTION: INVALID OPCODE.\n{:#?}",unsafe{&*stack_frame});
    loop{}
}

bitflags!{
    struct PageFaultErrorCode : u64 {
        const PROTECTION_VIOLATION = 1 << 0;
        const CAUSED_BY_WRITE = 1 << 1;
        const USER_MODE = 1 << 2;
        const MALFORMED_TABLE = 1 << 3;
        const INSTRUCTION_FETCH = 1 << 4;
    }
}

#[no_mangle]
extern "C" fn page_fault_handler(stack_frame : &ExceptionStackFrame, error_code : u64) -> ! {
    println!("\nPROCESSOR EXCEPTION: PAGE FAULT w/ Error Code = {:?}\n{:#?}",PageFaultErrorCode::from_bits(error_code).unwrap(),unsafe{&*stack_frame});
    loop{};
}