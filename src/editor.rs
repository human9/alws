extern crate ncurses;
use ncurses::*;

use view::LogView;
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

	pub fn capture_input(&self, lv: &mut LogView) {

		let mut ch = getch();
		while ch != 10 {
			match ch {
				_ => {
					let index = item_index(current_item(lv.menu)) as usize;
	                {
	                	let ei = {
	                    	let ref mut mission = lv.log.mission_list()[index];
	                    	mission.entries.len()-1
	                    };
	                   lv.log.mission_list()[index].entries[ei].entry_text.push(ch as u8 as char);
	                }
					lv.draw_window();
				},
			}
			ch = getch();
		}

	}

}