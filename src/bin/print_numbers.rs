// Not found!

/// Returns the max length of numbers in the given array
/// between decimal, hexadecimal and octal
fn get_max_len(arr: &Vec<u64>) -> usize {
    let max_dec = arr.iter().max().expect("The array is empty!");
    *[
        max_dec.to_string().len(),      // len of decimal
        format!("{:x}", max_dec).len(), // len of hexadecimal
        format!("{:o}", max_dec).len(), // len of octal
    ]
    .iter()
    .max() // Max's method returns a reference to the number,
    // so the base pointer return is set above.
    .unwrap()
}

fn main() {
    let arr: Vec<u64> = vec![60, 70, 80, 100, 200, 30000];
    let max_digit: usize = get_max_len(&arr);

    for num in arr {
        println!(
            "\
            \rNumber in decimal     {num:0>max_digit$}
            \rNumber in hexadecimal {num:0>max_digit$x}
            \rNumber in octal       {num:0>max_digit$o}
            ",
        );
    }
}
