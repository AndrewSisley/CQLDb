#[cfg(feature = "matrix")]
pub fn calculate_index(x: u64, y: u64, y_max: u64) -> u64 {
	((x - 1) * y_max) + (y - 1)
}

#[cfg(feature = "icosahedral")]
pub fn calculate_index(x: u64, y: u64, m: u64) -> u64 {
    let mut index = x - 1;

    let m1_bound = m + 1;
    let y1_max = if y < m1_bound { y } else { m1_bound };
    for y1 in 2..y1_max {
        index = index + (y1 - 1);
    }

    let m2_bound = (m * 2) + 1;
    let y2_max = if y < m2_bound { y } else { m2_bound };
    for _y2 in m..y2_max {
        index = index + m;
    }

    for y3 in m2_bound..y {
        index = index + (m - (y3 - m2_bound));
    }

    index
}
