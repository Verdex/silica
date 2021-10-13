
pub trait Parser<TInput, TOutput> {
    fn usable(&self, x : TInput) -> bool;
    fn parse(&self) -> TOutput;
}

