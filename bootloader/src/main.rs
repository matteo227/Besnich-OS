#![no_std]
#![no_main]

use besnich_crypto::{SignatureVerifier, HashValidator};

const BESNICH_ROOT_PUBKEY: [u8; 32] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let stage2 = load_sector(1..=64);
    verify_stage2_signature(&stage2);
    
    let kernel = load_kernel();
    
    let verifier = SignatureVerifier::new(&BESNICH_ROOT_PUBKEY);
    match verifier.verify(&kernel) {
        Ok(()) => {
            jump_to_kernel(kernel);
        }
        Err(e) => {
            besnich_secure_halt();
        }
    }
}

fn besnich_secure_halt() -> ! {
    unsafe {
        asm!("cli");
        write_nvram_log("[BESNICH] SECURE BOOT VIOLATION - KERNEL TAMPERED");
        loop { asm!("hlt"); }
    }
}
