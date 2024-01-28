use super::Skyapex;

#[skyapex_impl]
pub trait Utils {
    fn add(&mut self, left: i32, right: i32) -> i32;
    fn print_run_as_root(&mut self);
    fn clean_up(&mut self);
}

pub trait TestUtils {
    fn print(&mut self, str: String);
}

#[skyapex_impl]
trait TestUtilsImpl {
    fn echo(&mut self, str_ptr: i32);
}

impl TestUtils for Skyapex {
    fn print(&mut self, data: String) {
        let str_ptr = self.pass_string(data).unwrap();
        self.echo(str_ptr);
    }
}
