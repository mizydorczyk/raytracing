use std::{cmp::max, io::Write};

use raytracing::{
    ray::Ray,
    vec3::{Color3, Point3, Vec3},
};

fn write_color(color: &Color3) {
    let r = (color[0].clamp(0.0, 1.0) * 255.0).round() as u8;
    let g = (color[1].clamp(0.0, 1.0) * 255.0).round() as u8;
    let b = (color[2].clamp(0.0, 1.0) * 255.0).round() as u8;

    println!("{r} {g} {b}");
}

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> bool {
    let oc = *center - ray.origin;
    let a = Vec3::dot(ray.direction, ray.direction);
    let b = -2.0 * Vec3::dot(ray.direction, oc);
    let c = Vec3::dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    discriminant >= 0.0
}

fn ray_color(ray: &Ray) -> Color3 {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, &ray) {
        return Color3::new(0.0, 0.0, 1.0);
    }

    let unit_direction = Vec3::unit(ray.direction);
    let a = 0.5 * (unit_direction[1] + 1.0);

    (1.0 - a) * Color3::new(1.0, 1.0, 1.0) + a * Color3::new(0.5, 0.7, 1.0)
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1280;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let image_height = max(image_height, 1);

    // camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // vectors across the edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0); // horizontal
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0); // vertical

    // delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / image_width as f64; // horizontal edge
    let pixel_delta_v = viewport_v / image_height as f64; // vertical

    // location of the upper left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // render
    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        std::io::stderr().flush().unwrap();

        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;

            let ray = Ray::new(camera_center, ray_direction);
            let color = ray_color(&ray);

            write_color(&color);
        }
    }

    eprintln!("Done.")
}
