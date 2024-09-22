pub(crate) trait Question {
    fn question(&self) -> String;
    fn answer(&self) -> String;
}
