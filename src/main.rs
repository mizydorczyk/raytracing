use std::io::Write;

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

            let r = (r.clamp(0.0, 1.0) * 255.0).round() as u8;
            let g = (g.clamp(0.0, 1.0) * 255.0).round() as u8;
            let b = (b.clamp(0.0, 1.0) * 255.0).round() as u8;

            println!("{r} {g} {b}");
        }
    }

    eprintln!("Done.")
}
