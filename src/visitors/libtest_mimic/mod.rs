use crate::models::{Suite, World};

pub fn run<WorldImpl>(suite: Suite<WorldImpl>) -> std::process::ExitCode
where
    WorldImpl: World,
{
    let args = libtest_mimic::Arguments::from_args();
    let tests = suite.into();
    let conclusion = libtest_mimic::run(&args, tests);

    conclusion.exit_code()
}

impl<WorldImpl> From<Suite<WorldImpl>> for Vec<libtest_mimic::Trial> {
    fn from(suite: Suite<WorldImpl>) -> Self {
        todo!()
    }
}