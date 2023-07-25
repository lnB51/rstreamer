/* 
Main file of rstreamer project.
License: MIT
Github repo: https://github.com/lnB51/rstreamer
*/

#[allow(unused_variables)]

use rstreamer::parse_args;

use v4l::buffer::Type;
use v4l::io::mmap::Stream;
use v4l::io::traits::CaptureStream;
use v4l::video::Capture;
use v4l::Device;
use v4l::FourCC;
use v4l::Format;

use std::io::prelude::*;
use std::net::TcpListener;

fn main() {

    let config = parse_args();
    
    println!("Running on: http://{}:{}", config.host, config.port);

    // Add listener for http stream
    let listener = TcpListener::bind(format!("{}:{}", config.host, config.port)).expect("Failed to bind to address");

    // Open the video device
    let mut video_device = Device::with_path(config.device_path).expect("Failed to open device");

    // Parse the resolution
    let resolution_parts: Vec<&str> = config.resolution.split('x').collect();

    if resolution_parts.len() != 2 {
        println!("Error: Invalid resolution format. Use 'WIDTHxHEIGHT', e.g., 1920x1080.");
        return;
    }

    let width = resolution_parts[0].parse::<u32>().expect("Invalid resolution width.");
    let height = resolution_parts[1].parse::<u32>().expect("Invalid resolution height.");

    // Configure the video format and resolution
    let format = Format::new(width, height, FourCC::new(b"RGB3"));
    println!("Format in use:\n{}", format);
    video_device.set_format(&format).expect("Failed to write format");

    let mut v_stream = Stream::with_buffers(&mut video_device, Type::VideoCapture, 4)
        .expect("Failed to create buffer stream");

    let mut response_buf = Vec::new();
    let response = "HTTP/1.1 200 OK\r\nContent-Type: multipart/x-mixed-replace; boundary=frame\r\n\r\n";

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 1024];
                stream.read(&mut buffer).expect("Failed to read stream");

                if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
                    stream.write_all(response.as_bytes()).expect("Failed to write response");
                    stream.flush().expect("Failed to flush stream");

                    loop {
                        let (buf, _meta) = v_stream.next().unwrap();

                        let image_data = Vec::from(buf); // Function to get the updated image data

                        response_buf.clear();
                        response_buf.extend_from_slice(format!(
                            "--frame\r\nContent-Type: image/jpeg\r\nContent-Length: {}\r\n\r\n",
                            image_data.len()
                        ).as_bytes());

                        response_buf.extend_from_slice(&image_data);

                        stream
                            .write_all(&response_buf)
                            .expect("Failed to write response");
                        stream.flush().expect("Failed to flush stream");
                        
                    }
                } else {
                    let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
                    stream.write_all(response.as_bytes()).expect("Failed to write response");
                    stream.flush().expect("Failed to flush stream");
                }
            }
            Err(e) => {
                eprintln!("Failed to establish a connection: {}", e);
            }
        }
    }
}
