use super::Skyapex;

#[skyapex_impl]
pub trait Utils {
    fn add(&mut self, left: i32, right: i32) -> i32;
    fn print_run_as_root(&mut self);
    fn clean_up(&mut self);
}
