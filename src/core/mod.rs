
pub mod theme;
pub mod pattern;
pub mod desktop;
// pub mod postscript;
fn expand_path(origin_path:&str) ->String{
	let mut path = String::from(origin_path);
	path = shellexpand::tilde(&path).to_string();
	path = shellexpand::env(&path).unwrap().to_string();
	path
}


#[cfg(test)]
mod tests{
	// use super::pattern::Pattern;
	use super::theme::Theme;
	use super::desktop::Desktop;
	
	// #[test]
	// fn test_fill_pattern(){
	// 	let patterns = Pattern::get_patterns("jorge");
	// 	let pattern = patterns.into_iter().find(|pattern| pattern.get_name()=="kitty").unwrap().to_pattern();
		
	// 	let themes = Theme::get_themes();
	// 	let theme = themes.into_iter().find(|theme |theme.get_name()=="Nord" ).unwrap().to_theme();

	// 	// pattern.fill(&theme)
	// }
	// #[test]


	// #[test]
	// fn test_search_and_fill_pattern(){
	// 	let themes = Theme::get_themes();
	// 	let selected_file = themes.get(0).unwrap();
	// 	let selected_theme = selected_file.to_theme();

	// 	let patterns = Pattern::get_patterns("jorge");
	// 	let pattern = patterns.iter().find(|pattern| pattern.get_name()=="kitty").unwrap().to_pattern();

	// 	// pattern.fill(&selected_theme);
	// }

	#[test]
	fn test_apply_theme(){
		let desktops = Desktop::get_desktops();
		let desktop = desktops.into_iter().find(|desktop |desktop.get_name()=="jorge" ).unwrap();

		let themes = Theme::get_themes();
		let theme = themes.into_iter().find(|theme |theme.get_name()=="Nord" ).unwrap().to_theme();

		desktop.to_desktop().apply(&theme);
	}
}