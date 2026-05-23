#![no_std]
#![no_main]

use core::panic::PanicInfo;

const VGA_BUFFER: *mut u8 = 0xB8000 as *mut u8;

const GREEN: u8 = 0x02;
const LIGHT_GREEN: u8 = 0x0A;
const YELLOW: u8 = 0x0E;
const WHITE: u8 = 0x0F;

#[repr(u8)]
enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xA,
    LightCyan = 0xB,
    LightRed = 0xC,
    LightMagenta = 0xD,
    Yellow = 0xE,
    White = 0xF,
}

struct VgaWriter;

impl VgaWriter {
    fn clear_screen(&self, bg: u8) {
        let buffer = VGA_BUFFER;
        for i in 0..(80 * 25) {
            unsafe {
                *buffer.add(i * 2) = b' ';
                *buffer.add(i * 2 + 1) = bg << 4 | bg;
            }
        }
    }

    fn set_pixel(&self, x: u16, y: u16, fg: u8, bg: u8) {
        if x >= 80 || y >= 25 { return; }
        let offset = ((y as u32 * 80) + x as u32) * 2;
        unsafe {
            *VGA_BUFFER.add(offset as usize) = b' ';
            *VGA_BUFFER.add(offset as usize + 1) = bg << 4 | fg;
        }
    }

    fn write_string(&self, s: &str, x: u16, y: u16, fg: u8, bg: u8) {
        let mut cx = x;
        for byte in s.bytes() {
            if byte == b'\n' { break; }
            let offset = ((y as u32 * 80) + cx as u32) * 2;
            unsafe {
                *VGA_BUFFER.add(offset as usize) = byte;
                *VGA_BUFFER.add(offset as usize + 1) = bg << 4 | fg;
            }
            cx += 1;
            if cx >= 80 { break; }
        }
    }

    fn draw_shield(&self, start_x: u16, start_y: u16, fg: u8, bg: u8) {
        let shield = [
            "          BBBBBBBBBBB          ",
            "        BB  BBBBBBB  BB        ",
            "      BB  BB  BBB  BB  BB      ",
            "     BB  BBBB  B  BBBB  BB     ",
            "    BB  BBBBBB   BBBBBB  BB    ",
            "   BB  BBBBBBBB BBBBBBBB  BB   ",
            "  BB  BBBBBBBBBBBBBBBBBBB  BB  ",
            "  BB  BBBBBBBBBBBBBBBBBBB  BB  ",
            "   BB  BBBBBBBBBBBBBBBBB  BB   ",
            "    BB  BBBBBBBBBBBBBBB  BB    ",
            "     BB  BBBBBBBBBBBBB  BB     ",
            "      BB  BBBBBBBBBBB  BB      ",
            "        BB  BBBBBBB  BB        ",
            "          BBBBBBBBBBB          ",
        ];

        for (row, line) in shield.iter().enumerate() {
            for (col, ch) in line.bytes().enumerate() {
                if ch == b'B' {
                    self.set_pixel(start_x + col as u16, start_y + row as u16, fg, bg);
                }
            }
        }
    }

    fn draw_small_shield(&self, x: u16, y: u16, fg: u8, bg: u8) {
        let small = [
            "   BBBBBBB   ",
            "  B  BBB  B  ",
            " B BB B BB B ",
            " B BBBBBBB B ",
            "  B BBBBB B  ",
            "   B BBB B   ",
            "    B   B    ",
        ];
        for (row, line) in small.iter().enumerate() {
            for (col, ch) in line.bytes().enumerate() {
                if ch == b'B' {
                    self.set_pixel(x + col as u16, y + row as u16, fg, bg);
                }
            }
        }
    }
}

static VGA: VgaWriter = VgaWriter {};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    VGA.clear_screen(GREEN);

    VGA.draw_shield(28, 3, LIGHT_GREEN, GREEN);

    VGA.write_string("BESNICH OS", 29, 18, YELLOW, GREEN);
    VGA.write_string("Secure Boot Chain v1.0", 27, 19, WHITE, GREEN);

    for _ in 0..3000000 {
        unsafe { core::arch::asm!("nop"); }
    }

    load_stage2();
    jump_to_kernel();
}

fn load_stage2() {
    unsafe {
        core::arch::asm!(
            "mov ah, 0x02",
            "mov al, 0x01",
            "mov bx, 0x7E00",
            "mov cx, 0x0002",
            "mov dx, 0x0080",
            "int 0x13",
        );
    }
}

fn jump_to_kernel() -> ! {
    unsafe {
        core::arch::asm!(
            "jmp 0x0000:0x7E00",
            options(noreturn)
        );
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
