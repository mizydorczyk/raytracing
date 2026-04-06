use std::io::Write;

use raytracing::vec3::Color3;

fn write_color(color: &Color3) {
    let r = (color[0].clamp(0.0, 1.0) * 255.0).round() as u8;
    let g = (color[1].clamp(0.0, 1.0) * 255.0).round() as u8;
    let b = (color[2].clamp(0.0, 1.0) * 255.0).round() as u8;

    println!("{r} {g} {b}");
}

fn main() {
    let width = 256;
    let height = 256;

    println!("P3");
    println!("{width} {height}");
    println!("255");

    for j in 0..height {
        eprint!("\rScanlines remaining: {} ", height - j);
        std::io::stderr().flush().unwrap();

        for i in 0..width {
            let r = i as f64 / (width - 1) as f64;
            let g = j as f64 / (height - 1) as f64;
            let b = 0.0f64;

            let color = Color3::new(r, g, b);
            write_color(&color);
        }
    }

    eprintln!("Done.")
}
