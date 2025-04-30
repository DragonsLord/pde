const SEGMENTS: i32 = 20;

pub fn get_ascii_progress_bar(current_value: f32, max_value: f32) -> String {
    let segement_val = max_value / (SEGMENTS as f32);
    let full_bars = f32::round(current_value / segement_val) as i32;

    let mut str = String::with_capacity(SEGMENTS as usize);

    for _ in 0..full_bars {
        str.push('◼');
    }

    for _ in full_bars..SEGMENTS {
        str.push('▭');
    }

    return str;
}
