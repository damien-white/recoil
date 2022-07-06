pub struct Error<I> {
    /// Position of the error in the `Sequence`.
    pub input: I,
    /// Recoil error code.
    pub code: u16,
}
