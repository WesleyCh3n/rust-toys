fn main() {
    let raw_file =
        std::fs::File::open("./file-io/test.csv").expect("Can't open raw file");
    let reader_raw = std::io::BufReader::new(raw_file);
    // use std::io::BufRead;
    // reader_raw.lines().take(2).for_each(|l| {
    //     println!("{}", l.unwrap());
    // });
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(reader_raw);
    for result in rdr.records().take(2) {
        println!("{:?}", result);
    }
}
