pub struct Defer<F: Fn()>(F);

impl<F: Fn()> Drop for Defer<F> {
    fn drop(&mut self) {
        (self.0)()
    }
}
