mod injections;
use injections::{Dependencies, injectDependencies};

fn main() {
    let dependencies = unsafe { injectDependencies() };
}
