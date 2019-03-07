pub trait Tennis {
    fn won_point(&mut self, player_name: &str);
    fn get_score(&self) -> String;
}
