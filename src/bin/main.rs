use gtheme::app;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	app::Ui::default().start_ui()
}
