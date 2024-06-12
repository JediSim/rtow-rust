use image::{ImageBuffer, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use glam::f64::DVec3;

mod ray;
mod colors;

const IMAGE_WIDTH:u32 = 256;
const IMAGE_HEIGHT:u32 = 256;

fn main() {

    // ===================================================================== Create image
    let mut scale_rgb_image: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let pb = ProgressBar::new(IMAGE_HEIGHT as u64);
    // Style for progress bar
    pb.set_style(ProgressStyle::default_bar().template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("##-"));
    // final message
    pb.set_message("Creating image");

    // u32 for next calculation
    for j in 0..IMAGE_HEIGHT {
        pb.inc(1);
        for i in 0..IMAGE_WIDTH {
            colors::write_color(&mut scale_rgb_image, i, j, DVec3::new(i as f64 / (IMAGE_WIDTH - 1) as f64, j as f64 / (IMAGE_HEIGHT - 1) as f64, 0.25));
        }
    }
    pb.finish();

    scale_rgb_image.save("output.png").unwrap();
    // ===================================================================== Create image

    // ===================================================================== Ray tracing in scene
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let mut scale_wb_image: RgbImage = ImageBuffer::new(image_width, image_height);

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let camera_center = DVec3::new(0.0, 0.0, 0.0);

    let origin = DVec3::new(0.0,0.0,0.0);
    let horizontal = DVec3::new(viewport_width, 0.0, 0.0);
    let vertical = DVec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - DVec3::new(0.0, 0.0, focal_length);

    let pb_wb = ProgressBar::new(image_height as u64);
    pb_wb.set_style(ProgressStyle::default_bar().template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("##-"));
    pb_wb.set_message("Ray tracing");
    // Render
    for j in 0..image_height {
        pb_wb.inc(1);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = (image_height - j) as f64 / (image_height - 1) as f64;
            let ray = ray::Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);

            let pixel_color = colors::ray_color(&ray);
            colors::write_color(&mut scale_wb_image, i as u32, j as u32, pixel_color);
        }
    }
    pb_wb.finish();
    scale_wb_image.save("output_wb.png").unwrap();
    // ===================================================================== Ray tracing in scene
}
