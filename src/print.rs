pub fn main() {
    println!("{} days", 31);
    println!("{1}, {2}, {0}", 0, 1, 2);
    println!(
        "{swift}, {rust}, {python}",
        python = "python",
        rust = "rust",
        swift = "swift"
    );

    // numeric formatting
    println!("{:05}", 100); // 00100
    println!("{:#05x}", 16); // 0x010
    println!("{:#5o}", 16); // _0o20
    println!("{:#5b}", 16); // 0b10000
    println!("{:+#5b}", 16); // +0b10000

    // parameterize width with `{index}$`
    println!("{0:-<1$}", "x", 3); // x--
    println!("{:-^width$}", "x", width = 3); // -x-
    println!("{1:->0$}", 3, "x"); // --x
}
