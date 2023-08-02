use std::io::{stdout,Write};

//Implementing global mutables as a struct
struct Status {
    z_buf: [f32; SCREEN_WXH],
    buf: [char; SCREEN_WXH],
    a: f32,
    b: f32,
    c: f32,
}

const SCREEN_W: usize = 160;
const SCREEN_H: usize = 44;
const SCREEN_WXH: usize = (SCREEN_W * SCREEN_H) as usize;

const CUBE_W: u8 = 20;
const BG_ASCII: char = '.';
const CAM_DISTANCE: u8 = 100;
static HORIZONTAL_OFFSET: f32 = -2. * CUBE_W as f32;
const K1: u8 = 40;

static INCREMENT_SPEED: f32 = 0.6;

fn calculate_x(i: f32, j: f32, k: f32, status: &Status) -> f32 {
    return j * status.a.sin() * status.b.sin() * status.c.cos() - k * status.a.cos() * status.b.sin() * status.c.cos() +
    j * status.a.cos() * status.c.sin() + k * status.a.sin() * status.c.sin() + i * status.b.cos() * status.c.cos();
}

fn calculate_y(i: f32, j: f32, k: f32, status: &Status) -> f32 {
    return j * status.a.cos() * status.c.cos() + k * status.a.sin() * status.c.cos() -
    j * status.a.sin() * status.b.sin() * status.c.sin() + k * status.a.cos() * status.b.sin() * status.c.sin() -
    i * status.b.cos() * status.c.sin();
}

fn calculate_z(i: f32, j: f32, k: f32, status: &Status) -> f32 {
    return k * status.a.cos() * status.b.cos() - j * status.a.sin() * status.b.cos() + i * status.b.sin();
}

fn calculate_for_surface(cube_x: f32, cube_y: f32, cube_z: f32, ch: char, status: &mut Status) {
    let x = calculate_x(cube_x,cube_y,cube_z, status);
    let y = calculate_y(cube_x,cube_y,cube_z, status);
    let z = calculate_z(cube_x,cube_y,cube_z, status) + CAM_DISTANCE as f32;

    let ooz = 1. / z;
    let xp = (SCREEN_W as f32 / 2. + HORIZONTAL_OFFSET as f32 + K1 as f32 * ooz * x * 2.) as usize;
    let yp = (SCREEN_H as f32 / 2. + K1 as f32 * ooz * y) as usize;

    let idx = xp + yp * SCREEN_W;
    if idx < SCREEN_W * SCREEN_H && ooz > status.z_buf[idx] {
        status.z_buf[idx] = ooz;
        status.buf[idx] = ch;
    }
}

fn main() {
    print!("\x1b[?25l"); //hide cursor
    print!("\x1b[2J"); //clear screen

    let mut status = Status {
        z_buf: [0.; SCREEN_WXH],
        buf: [0 as char; SCREEN_WXH],
        a: 0.,
        b: 0.,
        c: 0.,
    };

    loop {
        status.buf.fill(BG_ASCII); //clear buffers
        status.z_buf.fill(0.);

        let mut cube_x = -1.*CUBE_W as f32;

        while cube_x < CUBE_W as f32 {
            let mut cube_y = -1.*CUBE_W as f32;
            while cube_y < CUBE_W as f32 {
                calculate_for_surface(cube_x, cube_y, -1.*CUBE_W as f32, '@', &mut status);
                calculate_for_surface(CUBE_W as f32, cube_y, cube_x, '$', &mut status);
                calculate_for_surface(-1.*CUBE_W as f32, cube_y, -1.*cube_x, '~', &mut status);
                calculate_for_surface(-1.*cube_x, cube_y, CUBE_W as f32, '#', &mut status);
                calculate_for_surface(cube_x, -1.*CUBE_W as f32, -cube_y, ';', &mut status);
                calculate_for_surface(cube_x, CUBE_W as f32, cube_y, '+', &mut status); 
                cube_y = cube_y + INCREMENT_SPEED;
            }
            cube_x = cube_x + INCREMENT_SPEED;
        }
        
        print!("\x1b[H");  // Move cursor to the top-left of the screen
        for k in 0..SCREEN_WXH {
            if (k % SCREEN_W) != 0 {
                print!("{}",status.buf[k]);
            } else {print!("{}", 10 as char)}
            let _ = stdout().flush();
        }
        
        status.a = status.a + 0.05;
        status.b = status.b + 0.05;
        status.c = status.c + 0.01;

    }
}