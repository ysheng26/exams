pub(crate) trait Question {
    fn question(&self, q_num: usize) -> String;
    fn answer(&self, q_num: usize) -> String;
}
