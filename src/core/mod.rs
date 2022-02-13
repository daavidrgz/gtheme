
pub mod theme;
pub mod pattern;

fn expand_path(origin_path:&str) ->String{
	let mut path = String::from(origin_path);
	path = shellexpand::tilde(&path).to_string();
	path = shellexpand::env(&path).unwrap().to_string();
	path
}


#[cfg(test)]
mod tests{
	use super::pattern::Pattern;
	use super::theme::Theme;
	
	#[test]
	fn test_fill_pattern(){
		let pattern = Pattern::from("./kitty.pattern");
		let theme = Theme::from("./themes/Vue.json");
		theme.fill_pattern(&pattern);
	}
	#[test]


	#[test]
	fn test_search_and_fill_pattern(){
		let themes = Theme::get_themes();
		let selected_file = themes.get(0).unwrap();
		let selected_theme = Theme::from(selected_file.get_path());
		let pattern = Pattern::from("./kitty.pattern");
		selected_theme.fill_pattern(&pattern);
	}
}