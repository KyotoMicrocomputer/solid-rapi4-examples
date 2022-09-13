/// Like [`FnMut`] but can be implemented on user types without unstable features.
pub trait FuncMut<Args> {
    type Output;

    fn call(&mut self, args: Args) -> Self::Output;
}

impl<T: FnMut() -> Output, Output> FuncMut<()> for T {
    type Output = Output;

    #[inline]
    fn call(&mut self, (): ()) -> Self::Output {
        self()
    }
}

impl<T: FnMut(A0) -> Output, A0, Output> FuncMut<(A0,)> for T {
    type Output = Output;

    #[inline]
    fn call(&mut self, (a0,): (A0,)) -> Self::Output {
        self(a0)
    }
}
