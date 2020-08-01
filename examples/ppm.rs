use ray_tracing::Scalar;
use std::fs::File;
use std::io::Write;

fn do_main() -> std::io::Result<()> {
	const IMAGE_WIDTH: usize = 256;
	const IMAGE_HEIGHT: usize = 256;

	let mut file = File::create("assets/hello_world.ppm")?;
	file.write_all(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())?;

	for h in (0..IMAGE_HEIGHT).rev() {
		print!("Scanlines remaining: {}\r", h);
		for w in 0..IMAGE_WIDTH {
			let r = w as Scalar / (IMAGE_WIDTH - 1) as Scalar;
			let g = h as Scalar / (IMAGE_HEIGHT - 1) as Scalar;
			let b: Scalar = 0.25;

			let ir: usize = (255.999 * r) as usize;
			let ig: usize = (255.999 * g) as usize;
			let ib: usize = (255.999 * b) as usize;

			file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
		}
	}
	Ok(())
}

fn main() {
	match do_main() {
		Ok(()) => println!("Successfully written .ppm file!"),
		Err(e) => println!("{}", e),
	}
}
