use std::time::Duration;
use std::thread;
use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::{ApiBackend, CameraFormat, CameraIndex, FrameFormat, RequestedFormat, RequestedFormatType, Resolution};
use nokhwa::{query, Camera};

fn print_buffer_hex(buffer: &[u8]) {
    // This function only for testing and will be removed soon
    for (i, byte) in buffer.iter().enumerate() {
        if i % 16 == 0 && i > 0 {
            println!();
        }
        print!("{:02X} ", byte);
    }
    println!();
}
fn main() {
    let cameras = query(ApiBackend::Auto).unwrap();
    cameras.iter().for_each(|cam| println!("{:?}", cam));
    let index = CameraIndex::Index(0);

    let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::Exact(CameraFormat::new(Resolution::new(640, 480), FrameFormat::MJPEG, 30)));

    let mut camera = Camera::new(index, requested).unwrap();
    camera.open_stream().unwrap();
    

    loop {
        let frame = camera.frame().unwrap();
        print_buffer_hex(frame.buffer());
        thread::sleep(Duration::from_millis(1000));
    }
}
