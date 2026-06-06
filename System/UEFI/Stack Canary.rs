static mut STACK_CANARY: u64 = 0xDEADBEEFCAFEBABE;

pub fn check_stack_canary() -> bool {
    unsafe { STACK_CANARY == 0xDEADBEEFCAFEBABE }
}
