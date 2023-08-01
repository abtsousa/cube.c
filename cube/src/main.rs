use std::io::{stdout,Write};
static mut Z_BUFFER: [f32; SCREEN_WXH] = [0.; SCREEN_WXH];
static mut BUFFER: [char; SCREEN_WXH] = [0 as char; SCREEN_W*SCREEN_H];

static mut A: f32 = 0.;
static mut B: f32 = 0.;
static mut C: f32 = 0.;

const SCREEN_W: usize = 160;
const SCREEN_H: usize = 44;
const SCREEN_WXH: usize = (SCREEN_W * SCREEN_H) as usize;


const CUBE_W: u8 = 20;
const BG_ASCII: char = '.';
const CAM_DISTANCE: u8 = 100;
static HORIZONTAL_OFFSET: f32 = -2. * CUBE_W as f32;
const K1: u8 = 40;

static INCREMENT_SPEED: f32 = 0.6;

unsafe fn calculate_x(i: f32, j: f32, k: f32) -> f32 {
    return j * A.sin() * B.sin() * C.cos() - k * A.cos() * B.sin() * C.cos() +
    j * A.cos() * C.sin() + k * A.sin() * C.sin() + i * B.cos() * C.cos();
}

unsafe fn calculate_y(i: f32, j: f32, k: f32) -> f32 {
    return j * A.cos() * C.cos() + k * A.sin() * C.cos() -
    j * A.sin() * B.sin() * C.sin() + k * A.cos() * B.sin() * C.sin() -
    i * B.cos() * C.sin();
}

unsafe fn calculate_z(i: f32, j: f32, k: f32) -> f32 {
    return k * A.cos() * B.cos() - j * A.sin() * B.cos() + i * B.sin();
}

unsafe fn calculate_for_surface(cube_x: f32, cube_y: f32, cube_z: f32, ch: char) {
    let x = calculate_x(cube_x,cube_y,cube_z);
    let y = calculate_y(cube_x,cube_y,cube_z);
    let z = calculate_z(cube_x,cube_y,cube_z) + CAM_DISTANCE as f32;

    let ooz = 1. / z;
    let xp = (SCREEN_W as f32 / 2. + HORIZONTAL_OFFSET as f32 + K1 as f32 * ooz * x * 2.) as usize;
    let yp = (SCREEN_H as f32 / 2. + K1 as f32 * ooz * y) as usize;

    let idx = xp + yp * SCREEN_W;
    if idx >= 0 && idx < SCREEN_W * SCREEN_H && ooz > Z_BUFFER[idx] {
        Z_BUFFER[idx] = ooz;
        BUFFER[idx] = ch;
    }
}

fn main() {
    print!("\x1b[?25l"); //hide cursor
    print!("\x1b[2J"); //clear screen
    loop {
        unsafe {
            BUFFER.fill(BG_ASCII); //clear buffers
            Z_BUFFER.fill(0.);
        }

        let mut cube_x = -1.*CUBE_W as f32;

        while cube_x < CUBE_W as f32 {
        let mut cube_y = -1.*CUBE_W as f32;
            while cube_y < CUBE_W as f32 { unsafe {
                calculate_for_surface(cube_x, cube_y, -1.*CUBE_W as f32, '@');
                calculate_for_surface(CUBE_W as f32, cube_y, cube_x, '$');
                calculate_for_surface(-1.*CUBE_W as f32, cube_y, -1.*cube_x, '~');
                calculate_for_surface(-1.*cube_x, cube_y, CUBE_W as f32, '#');
                calculate_for_surface(cube_x, -1.*CUBE_W as f32, -cube_y, ';');
                calculate_for_surface(cube_x, CUBE_W as f32, cube_y, '+'); 
            }
            cube_y = cube_y + INCREMENT_SPEED;
            }
            cube_x = cube_x + INCREMENT_SPEED;
        }

        print!("\x1b[H");  // Move cursor to the top-left of the scree
        for k in 0..SCREEN_WXH {
            if (k % SCREEN_W) != 0 {
                unsafe {print!("{}",BUFFER[k]);}
            } else {print!("{}", 10 as char)}
            let _ = stdout().flush();
        }

        unsafe {
            A = A + 0.05;
            B = B + 0.05;
            C = C + 0.01;
        }
    }
}