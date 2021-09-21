extern crate sdl2;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas};
use sdl2::video::{Window};


const RESOLUTION: (u32, u32) = (960, 540);
const WINDOW: (u32, u32) = (960, 540); 



fn main() {

    let sdl_context = sdl2::init().unwrap();
    let mut canvas = create_sdl_canvas(&sdl_context); 
    let texture_creator = canvas.texture_creator();

    let camera = start_camera(0, RESOLUTION);

    'main_loop: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Option::Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                _ => {}
            }
        }


        let frame = camera.capture().unwrap();

        let texture = texture_creator.load_texture_bytes(&frame[..]).unwrap();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
    }
}


fn create_sdl_canvas(sdl_context: &sdl2::Sdl) -> Canvas<Window> {
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
    let window = video_subsystem
        .window("Live video", WINDOW.0, WINDOW.1)
        .position_centered()
        .build()
        .map_err(|e| e.to_string()).unwrap();

    window.into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string()).unwrap()
}

fn start_camera(num: u32, resolution: (u32, u32)) -> rscam::Camera {

    let mut device_path = String::from("/dev/video");
    device_path.push_str(&num.to_string());

    let mut camera = rscam::new(&device_path).unwrap();
    for wformat in camera.formats() {
        let format = wformat.unwrap();
        println!("{:?}", format);
        println!("  {:?}", camera.resolutions(&format.format).unwrap());
    }


    camera
        .start(&rscam::Config {
            interval: (1, 30),
            resolution: resolution,
            format: b"MJPG",
            ..Default::default()
        })
        .unwrap();

    camera
}


