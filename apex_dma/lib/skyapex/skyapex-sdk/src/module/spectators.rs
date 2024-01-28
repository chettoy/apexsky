use super::Skyapex;

#[skyapex_impl]
trait SpecCheckImpl {
    fn _init_spec_checker(&mut self, local_ptr: i64);
    fn _tick_yew(&mut self, target_ptr: i64, yew: f32);
    fn _check_spec(&mut self, target_ptr: i64) -> f32;
    fn _is_spec(&mut self, target_ptr: i64) -> f32;
}

pub trait SpecCheck {
    fn init_spec_checker(&mut self, local_ptr: u64);
    fn tick_yew(&mut self, target_ptr: u64, yew: f32);
    fn is_spec(&mut self, target_ptr: u64) -> bool;
}

impl SpecCheck for Skyapex {
    fn init_spec_checker(&mut self, local_ptr: u64) {
        self._init_spec_checker(local_ptr as i64)
    }

    fn tick_yew(&mut self, target_ptr: u64, yew: f32) {
        self._tick_yew(target_ptr as i64, yew)
    }

    fn is_spec(&mut self, target_ptr: u64) -> bool {
        self._is_spec(target_ptr as i64) > 0.6
    }
}
