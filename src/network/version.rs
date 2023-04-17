#[derive(Copy, Clone)]
pub(super) struct Version(usize, usize, usize);

impl ToString for Version {
    fn to_string(&self) -> String {
        format!("{}.{}.{}", self.0, self.1, self.2)
    }
}
