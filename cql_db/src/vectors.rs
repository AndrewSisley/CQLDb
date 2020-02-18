pub fn calculate_index(x: u64, y: u64, y_max: u64) -> u64 {
	((x - 1) * y_max) + (y - 1)
}
