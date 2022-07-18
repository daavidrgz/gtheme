use gtheme::tui;
use gtheme::cli::{self, specification};

fn main() {
	let matches = specification::create_app(&vec![],&vec![], 
		&vec![], &vec![], &vec![]).get_matches();

	if matches.subcommand() == None {
		tui::start_tui();
	} else {
		cli::start_cli(matches);
	}
}
