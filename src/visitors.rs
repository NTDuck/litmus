use crate::Background;

trait Executor<Unit> {
    type Output;

    fn execute(&self, unit: Unit) -> Self::Output;
}

struct LibtestMimicExecutor;

impl<Given> Executor<Background<Given>> for LibtestMimicExecutor {
    type Output = ::core::result::Result<(), ::libtest_mimic::Failed>;
    
    fn execute(&self, background: Background<Given>) -> Self::Output {
        todo!()
    }
}
