// import the modules
mod data_capture;
// mod data_process;
// mod data_storage;
// mod data_query;
// mod data_visualize;
// mod common;

use std::path::Path;
use std::env;
use std::fs;
use image::{io::Reader as ImageReader, ImageBuffer, GrayImage, DynamicImage};
use tesseract::Tesseract;
use anyhow::{Result, Context};

fn main() -> Result<()> {
    // Get the current directory and print it
    let current_dir = env::current_dir()?;
    println!("Current directory: {}", current_dir.display());

    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Check if image filename was provided as argument
    let image_path = if args.len() > 1 
    {
        Path::new(&args[1]).to_path_buf()
    } 
    else 
    {
        println!("\nNo image filename provided as argument.");
        println!("Usage: cargo run -- <image_filename>");
        println!("Example: cargo run -- image.jpg");
        return Ok(());
    };

    println!("\nAttempting to read image from: {}", image_path.display());

    // Check if file exists
    if !image_path.exists() 
    {
        println!("Error: Image file not found!");
        return Ok(());
    }

    // Convert PathBuf to string
    let image_path_str = image_path.to_str().with_context(|| "Failed to convert path to string")?;

    // Read the image file directly into bytes
    let img_data = fs::read(&image_path).with_context(|| format!("Failed to read image file: {}", image_path.display()))?;

    println!("Successfully loaded image data. Size: {} bytes", img_data.len());
    println!("Initializing Tesseract OCR with German language support...");

    // Initialize Tesseract, set image, and get text in one chain
    let text = Tesseract::new(None, Some("deu"))
        .with_context(|| "Failed to initialize Tesseract. Make sure German language pack is installed.")?
        .set_image(image_path_str)
        .with_context(|| "Failed to set image in Tesseract")?
        .get_text()
        .with_context(|| "Failed to extract text")?;

    if text.trim().is_empty() 
    {
        println!("\nWarning: No text was extracted from the image!");
    } 
    else 
    {
        println!("\nExtracted text (German):");
        println!("{}", text);
    }

    Ok(())
}