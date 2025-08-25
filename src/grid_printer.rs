pub fn print_line(bits: u32) {
    // let num: u8 = 0b10110101; // Example number
    for i in 0..32 {
        let bit = (bits >> i) & 1;
        if bit == 1 {
            print!("▓")
        } else {
            print!("░");
        }
    }
    println!();
}

pub fn print_array(rows: &[u32]) {
    for u in rows {
        print_line(*u);
    }
}

pub fn set_px(rows: &mut [u32; 32], (x, y): (usize, usize), set_to: bool) {
    let row_mask = 2u32.pow(x as u32);
    let mut row = rows[y];
    if set_to {
        row = (row) | (row_mask as u32);
        rows[y] = row;
    } else {
        row = (row) & !(row_mask as u32);
        rows[y] = row;
    }
}
