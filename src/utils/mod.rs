use hex::decode;

pub fn hex_to_rgb(hex_color: &String) -> Option<(u8,u8,u8)> {
	if hex_color.len() != 7 || !hex_color.starts_with("#") {
		return None
	}

	match decode(&hex_color[1..]) {
		Ok(color) => Some((color[0], color[1], color[2])),
		Err(_) => return None
	}
}