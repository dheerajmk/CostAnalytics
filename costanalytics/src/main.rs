// import the modules
mod data_capture;
// mod data_process;
// mod data_storage;
// mod data_query;
// mod data_visualize;
// mod common;
use image::io::Reader as ImageReader;

fn main() {
    // Load the image from a file
    let img = ImageReader::open("lidl_receipt.jpg")
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");

    // Get the dimensions of the image
    let dimensions = img.dimensions();
    println!("Image dimensions: {:?}", dimensions);
}
