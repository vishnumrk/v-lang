mod frontend;
mod repl;
#[cfg(test)]
mod repl_test;
mod runtime;

fn main() {
    // repl_test::test_file::test_with_file()
    repl::run()
}

