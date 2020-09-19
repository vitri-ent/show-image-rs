use show_image::event;

#[show_image::main]
fn main() -> Result<(), String> {
	let args : Vec<_> = std::env::args().collect();
	if args.len() != 2 {
		return Err(format!("usage: {} IMAGE", args[0]));
	}

	let path = std::path::Path::new(&args[1]);
	let name = path.file_stem().and_then(|x| x.to_str()).unwrap_or("image");

	let image = image::open(path)
		.map_err(|e| format!("Failed to read image from {:?}: {}", path, e))?;

	let image_info = show_image::image_info(&image).map_err(|e| e.to_string())?;
	println!("{:#?}", image_info);

	let window = show_image::create_window("image", Default::default())
		.map_err(|e| e.to_string())?;
	window.set_image(name, image)
		.map_err(|e| e.to_string())?;

	// Wait for the window to be closed or Escape to be pressed.
	for event in window.event_channel().map_err(|e| e.to_string())? {
		if let event::WindowEvent::KeyboardInput(event) = event {
			if !event.is_synthetic && event.input.key_code == Some(event::VirtualKeyCode::Escape) && event.input.state.is_pressed() {
				println!("Escape pressed!");
				break;
			}
		}
	}

	Ok(())
}
