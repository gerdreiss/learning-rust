pub fn stats(verbose: bool, total_bytes: usize, last: bool) {
    if verbose {
        eprint!("\r{}", total_bytes);
        if last {
            eprintln!();
        }
    }
}
