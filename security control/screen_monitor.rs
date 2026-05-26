#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt::Write;

const FB_ADDR: *mut u32 = 0xFD000000 as *mut u32;
const FB_WIDTH: usize = 1024;
const FB_HEIGHT: usize = 768;
const SCAN_INTERVAL: u32 = 300_000;

const COLOR_BLACK: u32 = 0x00000000;
const COLOR_RED: u32 = 0x00FF0000;
const COLOR_DARK_RED: u32 = 0x00880000;
const COLOR_WHITE: u32 = 0x00FFFFFF;
const COLOR_YELLOW: u32 = 0x00FFFF00;
const COLOR_GREEN: u32 = 0x0000FF00;
const COLOR_CYAN: u32 = 0x0000FFFF;

const LOCK_NVRAM_ADDR: u32 = 0x0F000;
const LOCK_MAGIC: u32 = 0xB35E1C4B;
const JUSTIFICATION_ADDR: u32 = 0x0F100;
const JUSTIFICATION_MAX: usize = 256;
const RESPONSE_ADDR: u32 = 0x0F200;
const RESPONSE_MAX: usize = 128;

struct Framebuffer;

impl Framebuffer {
    fn write_pixel(&self, x: usize, y: usize, color: u32) {
        unsafe { *FB_ADDR.add(y * FB_WIDTH + x) = color; }
    }

    fn read_pixel(&self, x: usize, y: usize) -> u32 {
        unsafe { *FB_ADDR.add(y * FB_WIDTH + x) }
    }

    fn clear(&self, color: u32) {
        for y in 0..FB_HEIGHT {
            for x in 0..FB_WIDTH {
                self.write_pixel(x, y, color);
            }
        }
    }

    fn fill_rect(&self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        for py in y..(y + h).min(FB_HEIGHT) {
            for px in x..(x + w).min(FB_WIDTH) {
                self.write_pixel(px, py, color);
            }
        }
    }

    fn draw_char(&self, x: usize, y: usize, ch: u8, fg: u32) {
        let font: [[u8; 8]; 16] = [
            [0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00],
            [0x18,0x18,0x18,0x18,0x18,0x00,0x18,0x00],
            [0x66,0x66,0x66,0x00,0x00,0x00,0x00,0x00],
            [0x66,0xFF,0x66,0x66,0xFF,0x66,0x00,0x00],
            [0x18,0x3E,0x60,0x3C,0x06,0x7C,0x18,0x00],
            [0x62,0x66,0x0C,0x18,0x30,0x66,0x46,0x00],
            [0x3C,0x66,0x3C,0x38,0x67,0x66,0x3F,0x00],
            [0x18,0x18,0x18,0x00,0x00,0x00,0x00,0x00],
            [0x0C,0x18,0x30,0x30,0x30,0x18,0x0C,0x00],
            [0x30,0x18,0x0C,0x0C,0x0C,0x18,0x30,0x00],
            [0x00,0x66,0x3C,0xFF,0x3C,0x66,0x00,0x00],
            [0x00,0x18,0x18,0x7E,0x18,0x18,0x00,0x00],
            [0x00,0x00,0x00,0x00,0x00,0x18,0x18,0x30],
            [0x00,0x00,0x00,0x7E,0x00,0x00,0x00,0x00],
            [0x00,0x00,0x00,0x00,0x00,0x18,0x18,0x00],
            [0x06,0x0C,0x0C,0x18,0x30,0x30,0x60,0x00],
        ];

        let idx = ch as usize;
        if idx >= 128 { return; }

        for row in 0..16 {
            let glyph = font[row][idx % 8];
            if glyph == 0 { continue; }
            for col in 0..8 {
                if (glyph >> (7 - col)) & 1 != 0 {
                    let px = x * 8 + col;
                    let py = y * 16 + row;
                    if px < FB_WIDTH && py < FB_HEIGHT {
                        self.write_pixel(px, py, fg);
                    }
                }
            }
        }
    }

    fn draw_text(&self, s: &str, x: usize, y: usize, fg: u32) {
        let mut cx = x;
        for byte in s.bytes() {
            if byte == b'\n' || cx >= FB_WIDTH / 8 { break; }
            self.draw_char(cx, y, byte, fg);
            cx += 1;
        }
    }

    fn draw_text_big(&self, s: &str, x: usize, y: usize, fg: u32, scale: usize) {
        let mut cx = x;
        for byte in s.bytes() {
            if byte == b'\n' || cx >= FB_WIDTH / (8 * scale) { break; }
            self.fill_rect(cx * 8 * scale, y * 16 * scale, 8 * scale, 16 * scale, fg);
            cx += 1;
        }
    }
}

static FB: Framebuffer = Framebuffer {};

fn write_nvram(addr: u32, data: &[u8]) {
    for (i, &byte) in data.iter().enumerate() {
        unsafe {
            core::ptr::write_volatile((addr + i as u32) as *mut u8, byte);
        }
    }
}

fn read_nvram(addr: u32, len: usize) -> &'static [u8] {
    unsafe {
        core::slice::from_raw_parts(addr as *const u8, len)
    }
}

fn write_nvram_string(addr: u32, s: &str) {
    let bytes = s.as_bytes();
    let len = bytes.len().min(255);
    unsafe {
        core::ptr::write_volatile(addr as *mut u8, len as u8);
    }
    for (i, &byte) in bytes.iter().enumerate().take(len) {
        unsafe {
            core::ptr::write_volatile((addr + 1 + i as u32) as *mut u8, byte);
        }
    }
}

fn read_nvram_string(addr: u32) -> core::fmt::String {
    let len = unsafe { core::ptr::read_volatile(addr as *const u8) } as usize;
    let mut s = core::fmt::String::new();
    for i in 0..len {
        let c = unsafe { core::ptr::read_volatile((addr + 1 + i as u32) as *const u8) } as char;
        s.push(c);
    }
    s
}

fn is_system_locked() -> bool {
    let magic = unsafe {
        core::ptr::read_volatile(LOCK_NVRAM_ADDR as *const u32)
    };
    magic == LOCK_MAGIC
}

fn set_system_lock() {
    unsafe {
        core::ptr::write_volatile(LOCK_NVRAM_ADDR as *mut u32, LOCK_MAGIC);
    }
    for offset in 0x100..0x800 {
        unsafe {
            core::ptr::write_volatile((LOCK_NVRAM_ADDR + offset) as *mut u8, 0xFF);
        }
    }
}

fn check_bot_response() -> bool {
    let response = read_nvram(RESPONSE_ADDR, 4);
    response[0] == b'A' && response[1] == b'P' && response[2] == b'P' && response[3] == b'R'
}

fn read_justification() -> [u8; JUSTIFICATION_MAX] {
    let mut buf = [0u8; JUSTIFICATION_MAX];
    for (i, byte) in buf.iter_mut().enumerate() {
        *byte = unsafe {
            core::ptr::read_volatile((JUSTIFICATION_ADDR + i as u32) as *const u8)
        };
    }
    buf
}

fn input_text(prompt: &str, x: usize, y: usize, max_len: usize) -> [u8; 256] {
    let mut buf = [0u8; 256];
    let mut idx = 0;
    let mut cx = x;

    FB.draw_text(prompt, x, y, COLOR_YELLOW);
    cx += prompt.len();

    loop {
        let key = read_key();
        if key == 0x0D {
            buf[idx] = 0;
            break;
        }
        if key == 0x08 {
            if idx > 0 {
                idx -= 1;
                buf[idx] = 0;
                if cx > x {
                    cx -= 1;
                    FB.draw_char(cx, y, b' ', COLOR_BLACK);
                    FB.draw_char(cx, y, b'_', COLOR_CYAN);
                }
            }
            continue;
        }
        if key >= 0x20 && key < 0x7F && idx < max_len && cx < FB_WIDTH / 8 - 1 {
            buf[idx] = key;
            idx += 1;
            FB.draw_char(cx, y, key, COLOR_WHITE);
            if cx + 1 < FB_WIDTH / 8 {
                FB.draw_char(cx + 1, y, b'_', COLOR_CYAN);
            }
            cx += 1;
        }
    }

    buf
}

fn show_lock_screen_permanent() {
    loop {
        FB.clear(COLOR_BLACK);

        FB.fill_rect(0, 0, FB_WIDTH, 4, COLOR_RED);
        FB.fill_rect(0, FB_HEIGHT - 4, FB_WIDTH, 4, COLOR_RED);
        FB.fill_rect(0, 0, 4, FB_HEIGHT, COLOR_RED);
        FB.fill_rect(FB_WIDTH - 4, 0, 4, FB_HEIGHT, COLOR_RED);

        FB.draw_text_big("!!! DANGEROUS ACTIVITY DETECTED !!!", 3, 6, COLOR_RED, 3);

        FB.draw_text("BESNICH OS - SISTEMA BLOCCATO", 32, 15, COLOR_WHITE);
        FB.draw_text("Il monitor schermo ha rilevato attivita dannose.", 18, 18, COLOR_WHITE);
        FB.draw_text("Il sistema e stato bloccato definitivamente.", 19, 20, COLOR_WHITE);
        FB.draw_text("Anche reinstallando Besnich OS, questo blocco persiste.", 15, 22, COLOR_WHITE);
        FB.draw_text("Il blocco e salvato nella NVRAM - livello hardware.", 16, 24, COLOR_WHITE);

        FB.draw_text("PER SBLOCcare:", 35, 28, COLOR_YELLOW);
        FB.draw_text("1. Scrivi una giustificazione dettagliata qui sotto", 16, 30, COLOR_WHITE);
        FB.draw_text("2. Il bot BesnichSecure analizzera la tua risposta", 16, 32, COLOR_WHITE);
        FB.draw_text("3. Se il bot approva, il sistema verra sbloccato", 16, 34, COLOR_WHITE);

        FB.draw_text("GIUSTIFICAZIONE:", 30, 38, COLOR_CYAN);

        let justification = input_text("> ", 12, 41, 80);

        FB.draw_text("INVIO GIUSTIFICAZIONE AL BOT...", 25, 45, COLOR_YELLOW);
        write_nvram_string(JUSTIFICATION_ADDR, core::str::from_utf8(&justification).unwrap_or(""));

        FB.clear(COLOR_BLACK);
        FB.draw_text("IN ATTESA DI RISPOSTA DEL BOT...", 25, 20, COLOR_YELLOW);
        FB.draw_text("Il bot BesnichSecure sta analizzando la giustificazione.", 15, 22, COLOR_WHITE);
        FB.draw_text("Codice richiesta: BSN-", 30, 24, COLOR_WHITE);

        let req_id = unsafe { core::ptr::read_volatile(0x0F000 as *const u32) };
        FB.draw_text("Inviato. Attendi...", 35, 26, COLOR_CYAN);

        let mut attempts = 0;
        loop {
            for _ in 0..50000 {
                unsafe { core::arch::asm!("nop"); }
            }

            if check_bot_response() {
                FB.clear(COLOR_BLACK);
                FB.fill_rect(0, FB_HEIGHT / 2 - 60, FB_WIDTH, 120, 0x00004400);
                FB.draw_text("BOT BESNICHSecure - RICHIESTA APPROVATA", 18, 22, COLOR_GREEN);
                FB.draw_text("Giustificazione accettata. Sblocco in corso...", 18, 25, COLOR_WHITE);

                unsafe {
                    core::ptr::write_volatile(LOCK_NVRAM_ADDR as *mut u32, 0x00000000);
                }

                for i in 0..0x800 {
                    unsafe {
                        core::ptr::write_volatile((LOCK_NVRAM_ADDR + i) as *mut u8, 0x00);
                    }
                }

                for _ in 0..5000000 {
                    unsafe { core::arch::asm!("nop"); }
                }

                return;
            }

            attempts += 1;
            if attempts % 10 == 0 {
                FB.draw_text(".", 45 + attempts / 10, 28, COLOR_CYAN);
            }

            if attempts > 120 {
                FB.clear(COLOR_BLACK);
                FB.draw_text("BOT BESNICHSecure - RICHIESTA RESPINTA", 18, 18, COLOR_RED);
                FB.draw_text("La giustificazione fornita non e stata approvata.", 14, 20, COLOR_WHITE);
                FB.draw_text("Motivo: La spiegazione non e sufficiente a giustificare", 14, 22, COLOR_WHITE);
                FB.draw_text("l'attivita dannosa rilevata dallo screen monitor.", 16, 24, COLOR_WHITE);
                FB.draw_text("Riprova con una giustificazione piu dettagliata.", 16, 26, COLOR_WHITE);
                FB.draw_text("Premi un tasto per riprovare...", 25, 30, COLOR_YELLOW);

                read_key();
                break;
            }
        }
    }
}

fn analyze_region(x: usize, y: usize, w: usize, h: usize) -> u32 {
    let mut score = 0;
    for py in (y..y + h).step_by(8) {
        for px in (x..x + w).step_by(8) {
            if px >= FB_WIDTH || py >= FB_HEIGHT { continue; }
            let pixel = FB.read_pixel(px, py);
            let r = ((pixel >> 16) & 0xFF) as u8;
            let g = ((pixel >> 8) & 0xFF) as u8;
            let b = (pixel & 0xFF) as u8;

            if r > 200 && g < 50 && b < 50 { score += 3; }
            if r < 10 && g < 10 && b < 10 { score += 2; }
            if r > 240 && g > 240 && b > 240 { score += 4; }
        }
    }
    score
}

fn full_scan_anomaly() -> u32 {
    let mut total = 0;
    for ty in 0..(FB_HEIGHT / 64) {
        for tx in 0..(FB_WIDTH / 64) {
            total += analyze_region(tx * 64, ty * 64, 64, 64);
        }
    }
    total
}

fn check_screen_freeze() -> bool {
    let snapshot: [u32; 256] = [0; 256];
    for i in 0..256 {
        unsafe {
            core::ptr::write_volatile(
                (0xE0000 + i * 4) as *mut u32,
                FB.read_pixel(i * 4, FB_HEIGHT / 2)
            );
        }
    }
    for _ in 0..50000 {
        unsafe { core::arch::asm!("nop"); }
    }
    let mut matches = 0;
    for i in 0..256 {
        let px = unsafe { core::ptr::read_volatile((0xE0000 + i * 4) as *const u32) };
        if px == FB.read_pixel(i * 4, FB_HEIGHT / 2) {
            matches += 1;
        }
    }
    matches > 200
}

fn read_key() -> u8 {
    let key: u8;
    unsafe {
        core::arch::asm!(
            "mov ah, 0x01",
            "int 0x16",
            "jz no_key_rd",
            "mov ah, 0x00",
            "int 0x16",
            "jmp key_done_rd",
            "no_key_rd:",
            "mov al, 0",
            "key_done_rd:",
            out("al") key,
        );
    }
    key
}

#[no_mangle]
pub extern "C" fn screen_monitor_main() -> ! {
    if is_system_locked() {
        show_lock_screen_permanent();
    }

    loop {
        let mut threat = false;

        let r1 = analyze_region(0, 0, 200, 100);
        let r2 = analyze_region(400, 300, 200, 200);
        let r3 = analyze_region(800, 0, 224, 768);
        let r4 = analyze_region(0, 728, 1024, 40);
        let anomaly = full_scan_anomaly();
        let frozen = check_screen_freeze();

        if r1 > 50 || r2 > 50 || r3 > 50 || r4 > 50 { threat = true; }
        if anomaly > 500 { threat = true; }
        if frozen { threat = true; }

        if threat {
            set_system_lock();
            show_lock_screen_permanent();
        }

        for _ in 0..SCAN_INTERVAL {
            unsafe { core::arch::asm!("nop"); }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    FB.clear(COLOR_DARK_RED);
    FB.draw_text("BESNICH KERNEL PANIC", 35, 20, COLOR_WHITE);
    FB.draw_text("Screen monitor - errore irreversibile", 25, 24, COLOR_YELLOW);
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}
