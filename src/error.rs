pub struct Error<I> {
    /// Position of the error in the `Sequence`.
    pub input: I,
    /// Recoil error code.
    pub code: u16,
}

impl<I: AsRef<[u8]>> Error<I> {
    pub fn new(input: I, code: u16) -> Self {
        Self { input, code }
    }
}
