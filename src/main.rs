use std::f64::consts::*;

fn main() {
    let mut args = std::env::args();
    args.next();
    let path = args.nth(0)
    	.expect("Need path as zeroth args!");
    let xres = args.nth(0)
    	.expect("Need x resolution as first arg!")
    	.parse::<u32>()
    	.expect("x resolution needs to be an int!");
	let yres = args.nth(0)
    	.expect("Need y resolution as second arg!")
    	.parse::<u32>()
    	.expect("y resolution needs to be an int!");
	let xcenter = args.nth(0)
    	.expect("Need x center as third arg!")
    	.parse::<f64>()
    	.expect("x center needs to be a number!");
	let ycenter = args.nth(0)
    	.expect("Need y center as fourth arg!")
    	.parse::<f64>()
    	.expect("y center needs to be a number!");
	let width = args.nth(0)
		.expect("Need width as fifth arg!")
		.parse::<f64>()
    	.expect("Width needs to be a number!");
	let exp = args.nth(0)
		.expect("Need the exponent as sixth arg!")
		.parse::<f64>()
		.expect("Exponent needs to be a number!");
	let angle = args.nth(0)
		.unwrap_or("0".into())
		.parse::<f64>()
		.expect("Angle needs to be a number!");
    graph(path, xres, yres, xcenter, ycenter, width, angle, exp);
}

fn graph(path: String, xres: u32, yres: u32, xcent: f64, ycent: f64, width: f64, angle: f64, exp: f64) {
    let xbgn: f64 = xcent - width/2.0;
	let aspc: f64 = yres as f64 / xres as f64;
	let hght: f64 = aspc * width;
    let ybgn: f64 = ycent - hght/2.0;
	let xstp: f64 = width / xres as f64;
	let ystp: f64 = hght / yres as f64;

    let rot = num_complex::Complex64::from_polar(1.0, to_rad(angle));
    let rect = Rectangle::new()
    	.start(xbgn, ybgn)
    	.step(xstp, ystp);
	let buffer = map(rect, rot, exp, xres, yres);

	image::save_buffer_with_format(
    	path,
    	buffer.as_slice(),
    	xres, yres,
    	image::ColorType::Rgba8,
    	image::ImageFormat::Png
	).unwrap();
}

fn to_rad(degrees: f64) -> f64 {
	PI * degrees / 180.0
}

#[derive(Default)]
struct Rectangle {
	start: [f64;2],
	step: [f64;2],
}

impl Rectangle {
	fn new() -> Self {
		Self::default()
	}
	fn start(mut self, x: f64, y: f64) -> Self {
		self.start = [x, y];
		self
	}
	fn step(mut self, x: f64, y: f64) -> Self {
		self.step = [x, y];
		self
	}
}

fn map(
	rect: Rectangle,
	rot: num_complex::Complex64,
	exp: f64,
	xres: u32,
	yres: u32,
) -> Vec<u8> {
    const ITER: usize = 50;
    const THRESHOLD: f64 = 2.0;
	let stepx = rect.step[0];
	let stepy = rect.step[1];
	let start = num_complex::Complex64::new(rect.start[0], rect.start[1]);
	std::iter::successors(Some(start), |t| {
    	let mut c = *t;
    	c.im += stepy;
		Some(c)
	}).take(yres as usize)
	  .flat_map(|y| {
		std::iter::successors(Some(y), |t| {
			let mut c = *t;
			c.re += stepx;
			Some(c)
		}).take(xres as usize)
	})
	  .map(|c| series(c * rot, exp, ITER, THRESHOLD))
	  .flat_map(|p| pick_color(p))
	  .collect()
}

fn series(
    c: num_complex::Complex64,
    exp: f64,
    iter: usize,
    threshold: f64,
) -> f64 {
    let mut z = num_complex::Complex64::new(0.0, 0.0);
    for i in 0..iter {
		z = z.powf(exp) + c;
		if z.norm() > threshold {
    		let proximity = 1.0 - (i as f64/iter as f64);
			return proximity;
		}
    }
    0.0
}

fn pick_color(proximity: f64) -> [u8; 4] {
    //6F8F72
    const BLACK: [u8; 3] = [0x27, 0x29, 0x32];
    let alpha = (255.0 * (1.0 - proximity)) as u8;
    [BLACK[0], BLACK[1], BLACK[2], alpha]
}
