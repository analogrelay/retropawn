use std::time::{Instant, Duration};

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

const EXPECTED_LIB_RETRO_VERSION: u32 = 1;

// pub type EnvironmentCallback = unsafe extern "C" fn(command: libc::c_uint, data: *mut libc::c_void) -> bool;

// unsafe extern "C" fn log_callback(level: LogLevel, fmt: *const libc::c_char) {
//     let fmt = CStr::from_ptr(fmt).to_str().unwrap();
//     let levelStr = match level {
//         LogLevel::Debug => "DEBUG",
//         LogLevel::Info => "INFO",
//         LogLevel::Warn => "WARN",
//         LogLevel::Error => "ERROR",
//         _ => "UNKN",
//     };
//     println!("[{}] {}", levelStr, fmt);
// }


// unsafe extern "C" fn libretro_environment_callback(command: c_uint, return_data: *mut c_void) -> bool {
//     match command {
//         ENVIRONMENT_SET_VARIABLES => {
//             // Variables is a list of key value pairs, with the last pair being a null key
//             let mut variable = return_data as *mut Variable;
//             while !(*variable).key.is_null() {
//                 let key = CStr::from_ptr((*variable).key).to_str().unwrap();
//                 let value = CStr::from_ptr((*variable).value).to_str().unwrap();
//                 println!("Var: {} = {}", key, value);
//                 variable = variable.add(1);
//             }
//             return true;
//         }
//         ENVIRONMENT_SET_SUPPORT_NO_GAME => {
//             *(return_data as *mut bool) = true;
//             return true;
//         },
//         ENVIRONMENT_GET_LOG_INTERFACE => {
//             let cb = LogCallback {
//                 log: log_callback,
//             };
//             *(return_data as *mut LogCallback) = cb;
//             println!("Log interface set");
//             return true;
//         },
//         _ => {
//             println!("Unknown command: {}", command);
//         }
//     }

//     false
// }

fn load_core() {
    let core = libretropawn::Core::load("cores/snes9x_libretro.dylib").expect("Failed to load core");
    let api_version = core.get_api_version();
    println!("API Version: {}", api_version);
    if api_version != EXPECTED_LIB_RETRO_VERSION {
        panic!("The Core has been compiled with a LibRetro API that is unexpected, we expected version to be: {} but it was: {}", EXPECTED_LIB_RETRO_VERSION, api_version)
    }
}

fn main() {
    load_core();

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Rust Game",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut x: usize = 0;
    let mut y: usize = 0;

    let mut fps_timer = Instant::now();
    let mut fps_counter = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear the buffer to black
        buffer.fill(0x00000000);

        // Calculate fps
        fps_counter += 1;
        let elapsed = fps_timer.elapsed();
        if elapsed >= Duration::from_secs(1) {
            let fps = fps_counter as f64 / elapsed.as_secs_f64();
            window.set_title(&format!("Rust Game (FPS: {:.2})", fps));
            fps_counter = 0;
            fps_timer = Instant::now();
        }

        // Move the pixel when the arrow keys are pressed
        if window.is_key_down(Key::Left) && x > 0 {
            x -= 1;
        }
        if window.is_key_down(Key::Right) && x < WIDTH - 1 {
            x += 1;
        }
        if window.is_key_down(Key::Up) && y > 0 {
            y -= 1;
        }
        if window.is_key_down(Key::Down) && y < HEIGHT - 1 {
            y += 1;
        }

        // Set the pixel to blue
        buffer[y * WIDTH + x] = 0x0000FFFF;

        // Update the window buffer and display the changes
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
