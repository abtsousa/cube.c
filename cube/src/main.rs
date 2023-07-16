static A: f64 = 0.;
static B: f64 = 0.;
static C: f64 = 0.;
static cube_w: f64 = 20.;
static screen_w: u8 = 160;
static screen_h: u8 = 44;
static screen_wxh: u32 = (screen_w as u32) * (screen_h as u32);
static z_buffer: [char; screen_wxh] = [0; screen_wxh];
static buffer = [screen_w*screen_h];
static bg_ascii = '.';
static cam_distance = 100;
static horizontal_offset: i32;
static k1: i32 = 40;

static x: i32;
static y: i32;
static z: i32;
static ooz: i32;
static xp: u8;
static yp: u8;
static idx: u8;


fn main() {} 

fn calculate_x(i: i32, j: i32, k: i32) -> i32 {
    return j * A.sin() * B.sin() * C.cos() - k * A.cos() * B.sin() * C.cos() +
    j * A.cos() * C.sin() + k * A.sin() * C.sin() + i * B.cos() * C.cos();
}
