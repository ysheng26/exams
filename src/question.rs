pub(crate) trait Question {
    fn question(&self, id: usize) -> String;
    fn answer(&self, id: usize) -> String;
}
