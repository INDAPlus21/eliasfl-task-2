use std::{cmp::min, io};

fn main() {
    // Read one line from input
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading line");
    // Split by whitespace into `u16` parts
    let mut parts = input.trim().split_whitespace().map(|part| {
        part.parse::<u16>()
            .expect("What you entered is not a valid positive whole number")
    });

    // Get first two numbers as rows and columns
    let rows = parts.next().expect("No row number supplied");
    let cols = parts.next().expect("No column number supplied");

    // Iterate over rows
    for r in 1..=rows {
        // Iterate over columns for each row
        for c in 1..=cols {
            // Vertical distance is minimum of row number and row number from bottom + 1
            let vertical_dist = min(r, rows - r + 1);
            // Horizontal distance is minimum of column number and column number from right + 1
            let horizontal_dist = min(c, cols - c + 1);
            // Total distance is the minimum of vertical and horizontal distance
            let distance = min(vertical_dist, horizontal_dist);
            let distance = distance.to_string();
            print!(
                "{}",
                // Print '.' if string of distance has more than one character
                if distance.chars().count() > 1 {
                    String::from(".")
                } else {
                    distance
                }
            );
        }
        // Print new line when continuing to next row
        println!();
    }
}
