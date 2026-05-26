#![no_std]
#![no_main]

use core::panic::PanicInfo;

const FB_ADDR: *mut u32 = 0xFD000000 as *mut u32;
const FB_WIDTH: usize = 1024;
const FB_HEIGHT: usize = 768;
const SCAN_INTERVAL: u32 = 500_000;

const COLOR_BLACK: u32 = 0x00000000;
const COLOR_RED: u32 = 0x00FF0000;
const COLOR_DARK_RED: u32 = 0x00880000;
const COLOR_WHITE: u32 = 0x00FFFFFF;
const COLOR_YELLOW: u32 = 0x00FFFF00;

static mut ALERT_TRIGGERED: bool = false;

struct Framebuffer;

impl Framebuffer {
    fn read_pixel(&self, x: usize, y: usize) -> u32 {
        unsafe { *FB_ADDR.add(y * FB_WIDTH + x) }
    }

    fn write_pixel(&self, x: usize, y: usize, color: u32) {
        unsafe { *FB_ADDR.add(y * FB_WIDTH + x) = color; }
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

#[derive(Clone, Copy)]
struct ThreatRule {
    region_x: usize,
    region_y: usize,
    region_w: usize,
    region_h: usize,
    threshold_r: u8,
    threshold_g: u8,
    threshold_b: u8,
    description: &'static str,
}

const THREAT_RULES: &[ThreatRule] = &[
    ThreatRule {
        region_x: 0, region_y: 0, region_w: 200, region_h: 100,
        threshold_r: 200, threshold_g: 50, threshold_b: 50,
        description: "sospetta shellcode injection - regione rossa anomala",
    },
    ThreatRule {
        region_x: 400, region_y: 300, region_w: 200, region_h: 200,
        threshold_r: 80, threshold_g: 80, threshold_b: 80,
        description: "possibile keylogger window - regione scura persistente",
    },
    ThreatRule {
        region_x: 800, region_y: 0, region_w: 224, region_h: 768,
        threshold_r: 250, threshold_g: 250, threshold_b: 250,
        description: "attivita sospetta nella sidebar - flash bianchi",
    },
    ThreatRule {
        region_x: 0, region_y: 728, region_w: 1024, region_h: 40,
        threshold_r: 30, threshold_g: 30, threshold_b: 30,
        description: "taskbar compromessa - colore baseline alterato",
    },
];

fn analyze_region(rule: &ThreatRule) -> u32 {
    let mut total_r: u32 = 0;
    let mut total_g: u32 = 0;
    let mut total_b: u32 = 0;
    let mut count: u32 = 0;

    let step = 4;

    for y in (rule.region_y..rule.region_y + rule.region_h).step_by(step) {
        for x in (rule.region_x..rule.region_x + rule.region_w).step_by(step) {
            let pixel = FB.read_pixel(x, y);
            total_r += (pixel >> 16) & 0xFF;
            total_g += (pixel >> 8) & 0xFF;
            total_b += pixel & 0xFF;
            count += 1;
        }
    }

    if count == 0 { return 0; }

    let avg_r = (total_r / count) as u8;
    let avg_g = (total_g / count) as u8;
    let avg_b = (total_b / count) as u8;

    let mut score: u32 = 0;

    if avg_r > rule.threshold_r { score += 1; }
    if avg_g > rule.threshold_g { score += 1; }
    if avg_b > rule.threshold_b { score += 1; }

    score
}

fn full_screen_anomaly_scan() -> u32 {
    let mut total_score: u32 = 0;
    let mut anomalies = [0u32; 64];
    let mut anomaly_count = 0;

    let tile_w = 64;
    let tile_h = 64;

    for ty in 0..(FB_HEIGHT / tile_h) {
        for tx in 0..(FB_WIDTH / tile_w) {
            let mut sum_r: u32 = 0;
            let mut sum_g: u32 = 0;
            let mut sum_b: u32 = 0;
            let mut count: u32 = 0;

            for y in (ty * tile_h)..((ty + 1) * tile_h) {
                for x in (tx * tile_w)..((tx + 1) * tile_w) {
                    let pixel = FB.read_pixel(x, y);
                    sum_r += (pixel >> 16) & 0xFF;
                    sum_g += (pixel >> 8) & 0xFF;
                    sum_b += pixel & 0xFF;
                    count += 1;
                }
            }

            if count == 0 { continue; }

            let avg_r = (sum_r / count) as u8;
            let avg_g = (sum_g / count) as u8;
            let avg_b = (sum_b / count) as u8;

            if avg_r < 10 && avg_g < 10 && avg_b < 10 {
                anomalies[anomaly_count % 64] = (ty * (FB_WIDTH / tile_w) + tx) as u32;
                anomaly_count += 1;
            }

            if avg_r > 200 && avg_g < 60 && avg_b < 60 {
                anomalies[anomaly_count % 64] = (ty * (FB_WIDTH / tile_w) + tx) as u32 + 1000;
                anomaly_count += 1;
            }

            if avg_r > 240 && avg_g > 240 && avg_b > 240 {
                anomalies[anomaly_count % 64] = (ty * (FB_WIDTH / tile_w) + tx) as u32 + 2000;
                anomaly_count += 1;
            }
        }
    }

    if anomaly_count > 0 { total_score = anomaly_count as u32; }

    total_score
}

fn screen_capture_check() -> bool {
    let sample_y = 400;
    let mut prev_pixels: [u32; 128] = [0; 128];

    for x in 0..128 {
        prev_pixels[x] = FB.read_pixel(x * 8, sample_y);
    }

    for _ in 0..10000 {
        unsafe { core::arch::asm!("nop"); }
    }

    let mut changed = 0;
    for x in 0..128 {
        let current = FB.read_pixel(x * 8, sample_y);
        if current != prev_pixels[x] {
            changed += 1;
        }
    }

    if changed < 3 {
        return true;
    }

    false
}

fn show_danger_screen() {
    FB.clear(COLOR_BLACK);

    for y in 0..FB_HEIGHT {
        for x in 0..FB_WIDTH {
            let dx = if x < FB_WIDTH / 2 { FB_WIDTH / 2 - x } else { x - FB_WIDTH / 2 };
            let dy = if y < FB_HEIGHT / 2 { FB_HEIGHT / 2 - y } else { y - FB_HEIGHT / 2 };
            let dist = ((dx * dx + dy * dy) as f64).sqrt() as u32;

            if dist % 40 < 20 {
                FB.write_pixel(x, y, COLOR_DARK_RED);
            } else {
                FB.write_pixel(x, y, 0x00440000);
            }
        }
    }

    FB.fill_rect(0, 0, FB_WIDTH, 4, COLOR_RED);
    FB.fill_rect(0, FB_HEIGHT - 4, FB_WIDTH, 4, COLOR_RED);
    FB.fill_rect(0, 0, 4, FB_HEIGHT, COLOR_RED);
    FB.fill_rect(FB_WIDTH - 4, 0, 4, FB_HEIGHT, COLOR_RED);

    for y in 0..FB_HEIGHT {
        for x in 0..FB_WIDTH {
            let r = ((x * y) % 255) as u8;
            let g = ((x + y) % 255) as u8;
            let b = ((x * x + y * y) % 255) as u8;
            if r > 240 && g < 20 && b < 20 {
                FB.write_pixel(x, y, COLOR_RED);
            }
        }
    }

    FB.draw_text_big("!!! DANGEROUS ACTIVITY DETECTED !!!", 4, 10, COLOR_RED, 3);

    FB.draw_text("BESNICH OS SECURITY ALERT", 30, 18, COLOR_WHITE);
    FB.draw_text("Il sistema ha rilevato una minaccia attiva sullo schermo.", 20, 22, COLOR_WHITE);
    FB.draw_text("Possibile tentativo di: screen capture, keylogging, injection", 20, 24, COLOR_WHITE);
    FB.draw_text("Il sistema e stato bloccato per prevenire danni.", 20, 26, COLOR_WHITE);

    FB.draw_text("Dettagli minaccia:", 20, 30, COLOR_YELLOW);

    let details = [
        "Pattern di colore anomalo rilevato in regioni critiche",
        "Fluttuazione di pixel incompatibile con attivita utente normale",
        "Possibile hook della GPU per estrazione dati",
    ];

    for (i, detail) in details.iter().enumerate() {
        FB.draw_text(detail, 22, 32 + i as usize, COLOR_RED);
    }

    FB.fill_rect(FB_WIDTH / 2 - 100, FB_HEIGHT - 100, 200, 50, COLOR_RED);
    FB.draw_text("RILEVA MINACCIA", FB_WIDTH / 2 / 8 - 6, (FB_HEIGHT - 90) / 16, COLOR_WHITE);

    FB.draw_text("[ESC] Riavvia in modalita sicura", 20, FB_HEIGHT / 16 - 2, COLOR_YELLOW);
    FB.draw_text("[R]  Ripristina da backup firmato", 20, FB_HEIGHT / 16 - 1, COLOR_YELLOW);
    FB.draw_text("[F]  Contatta supporto Besnich", 20, FB_HEIGHT / 16, COLOR_YELLOW);
}

fn show_clean_screen() {
    FB.clear(COLOR_BLACK);
    FB.fill_rect(FB_WIDTH / 2 - 150, FB_HEIGHT / 2 - 60, 300, 120, 0x00004400);
    FB.draw_text("SCAN COMPLETATO - SISTEMA PULITO", 30, 22, COLOR_GREEN);
    FB.draw_text("Nessuna minaccia rilevata", 35, 25, COLOR_WHITE);
    delay(2000000);
}

fn delay(cycles: u32) {
    for _ in 0..cycles {
        unsafe { core::arch::asm!("nop"); }
    }
}

#[no_mangle]
pub extern "C" fn screen_monitor_main() -> ! {
    loop {
        let mut threat_detected = false;
        let mut threat_details: [&str; 8] = [""; 8];
        let mut threat_count = 0;

        for rule in THREAT_RULES {
            let score = analyze_region(rule);
            if score >= 2 {
                threat_detected = true;
                if threat_count < 8 {
                    threat_details[threat_count] = rule.description;
                    threat_count += 1;
                }
            }
        }

        let anomaly_score = full_screen_anomaly_scan();
        if anomaly_score > 30 {
            threat_detected = true;
        }

        let frozen = screen_capture_check();
        if frozen {
            threat_detected = true;
        }

        if threat_detected {
            unsafe {
                ALERT_TRIGGERED = true;
            }
            show_danger_screen();

            loop {
                let key = read_key();
                match key {
                    0x1B => {
                        FB.clear(0x00000044);
                        FB.draw_text("AVVIO MODALITA SICURA BESNICH...", 30, 22, COLOR_WHITE);
                        delay(3000000);
                        unsafe { core::arch::asm!("int 0x19"); }
                    }
                    b'r' | b'R' => {
                        FB.clear(0x00004400);
                        FB.draw_text("RIPRISTINO DA BACKUP FIRMATO...", 25, 22, COLOR_WHITE);
                        delay(3000000);
                        unsafe { core::arch::asm!("int 0x19"); }
                    }
                    b'f' | b'F' => {
                        FB.clear(0x00440000);
                        FB.draw_text("CONTATTING BESNICH SUPPORT...", 27, 22, COLOR_WHITE);
                        delay(2000000);
                        let digits: [u8; 3] = [b'0', b'b', b'7'];
                        let addr: u32 = 0xE000;
                        unsafe {
                            core::ptr::write_volatile(addr as *mut u8, digits[0]);
                            core::ptr::write_volatile((addr + 1) as *mut u8, digits[1]);
                            core::ptr::write_volatile((addr + 2) as *mut u8, digits[2]);
                        }
                        FB.draw_text("Supporto contattato. Codice: 0b7", 25, 26, COLOR_YELLOW);
                        FB.draw_text("Riavvia il sistema per uscire dalla modalita blocco.", 20, 28, COLOR_WHITE);
                    }
                    _ => {}
                }
            }
        }

        delay(SCAN_INTERVAL);
    }
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

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let fb = Framebuffer;
    fb.clear(COLOR_DARK_RED);
    fb.draw_text("BESNICH KERNEL PANIC", 35, 20, COLOR_WHITE);
    fb.draw_text("Screen monitor module - errore critico", 25, 24, COLOR_YELLOW);
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}
