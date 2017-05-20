/* Interrupt service routine module */

/* Assembly function for setting up an interrupt gate for a service routine */
#[allow(dead_code)]
extern "C" {
    fn set_isr(interrupt_num:u64, function_address:u64);
}

/* Setup the interrupt gates */
pub fn init_isr(){

}

