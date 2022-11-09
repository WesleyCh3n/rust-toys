fn single_num_tbl(n: isize, i: isize) {
    if i > 9 {
        return;
    }
    print!("{:>2} x {:>2} = {:>2};", n, i, n * i);
    return single_num_tbl(n, i + 1);
}

fn mult_tbl(n: isize, i: isize) {
    if n > 9 {
        return;
    }
    single_num_tbl(n, i);
    println!("");
    return mult_tbl(n + 1, i);
}

fn main() {
    mult_tbl(1, 1);
}
