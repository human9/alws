/// Simple text buffer
pub struct Buffer {
	lines: Vec<String>,
}

impl Buffer {

	pub fn new() -> Self {
		Buffer {
			lines: Vec::new(),
		}
	}

}