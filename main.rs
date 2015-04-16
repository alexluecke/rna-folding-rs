use std::cmp;
use std::char;

fn main() {

    let rna_string: Vec<char> = "ACGCGGUACUGAACG"
        .to_string()
        .chars()
        .collect();

    let n = rna_string.len();
    let mut m = [[0u64; 15]; 15];

    //println!("matrix: {}", m[1][1]);
    //println!("beta: {}", beta(&2, &3, &rna_string));
    //println!("beta: {}", beta(&0, &2, &rna_string));
    //println!("beta: {}", beta(&1, &4, &rna_string));

    let mut max_v: u64 = 0;
    for j in (1..n) {
        let mut i = j-1;
        while i > 0 {

            let b = beta(&i, &j, &rna_string);
            max_v = cmp::max(b, m[i+1][j-1]);

            for k in (i..j) {
                max_v = cmp::max(max_v, m[i][k]+ m[k+1][j]);
            }
            i -= 1;
        }

        m[i][j] = max_v;

    }

    //for j in (2..n) {
        //for i in (1..j) {
            //m[i][j] = cmp::max(m[i+1][j], m[i][j]);
        //}
    //}


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
    if dist <= 1 {
        return 0
    }

    let mut i = 0;
    let mut a: char = 'x';
    for val in s.iter() {
        if i == *x {
            a = *val;
            break;
        }
        i += 1;
    }

    i = 0;
    let mut b: char = 'x';
    for val in s.iter() {
        if i == *y {
            b = *val;
            break;
        }
        i += 1;
    }

    //println!("v[x]: {}, v[y]: {}", a, b);
    match a {
        'C' => if b == 'G' { 1 } else { 0 },
        'G' => if b == 'C' { 1 } else { 0 },
        'A' => if b == 'U' { 1 } else { 0 },
        'U' => if b == 'A' { 1 } else { 0 },
        _ => 0
    }
}
