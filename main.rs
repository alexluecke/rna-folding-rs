use std::cmp;

fn main() {

    let rna_string: Vec<char> = "ACGCGGUACUGAACG"
        .to_string()
        .chars()
        .collect();

    let n = rna_string.len();
    let mut m = [[0u64; 15]; 15];

    for j in (1..n) {
        for i in (0..j) {
            m[i][j] = cmp::max(
                m[i+1][j-1] + beta(&i, &j, &rna_string),
                m[i][j-1]
            );
        }

        for i in (1..j).rev() {
            m[i][j] = cmp::max(
                m[i+1][j],
                m[i][j]
            );
            for k in (i+1..j).rev() {
                m[i][j] = cmp::max(
                    m[i][j],
                    m[i][k-1]+m[k][j]
                );
            }
        }
    }

    for row in m.iter() {
        for item in row.iter() {
            print!("{} ", item);
        }
        println!("");
    }
}

fn beta(x: &usize, y: &usize, s: &Vec<char>) -> u64 {

    // Can't be adjacent chars
    let p: isize = *x as isize;
    let q: isize = *y as isize;

    let dist = (p-q).abs();

    if dist <= 1 { return 0 }

    let a: char = get_char_from_idx(x, s);
    let b: char = get_char_from_idx(y, s);

    match a {
        'C' => if b == 'G' { 1 } else { 0 },
        'G' => if b == 'C' { 1 } else { 0 },
        'A' => if b == 'U' { 1 } else { 0 },
        'U' => if b == 'A' { 1 } else { 0 },
        _ => 0
    }
}

fn get_char_from_idx(idx: &usize, v: &Vec<char>) -> char {
    let mut i = 0;
    let mut c: char = ' ';
    for val in v.iter() {
        if i == *idx {
            c = *val;
            break;
        }
        i += 1;
    }
    return c
}
