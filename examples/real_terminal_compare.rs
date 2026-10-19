mod unix;

fn main() {
    #[cfg(unix)]
    unix::real_terminal_compare::main();
}
