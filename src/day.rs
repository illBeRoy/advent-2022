pub trait Day {
    fn title(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn task_1(&self) -> String;
    fn task_2(&self) -> String;
}
