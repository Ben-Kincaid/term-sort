pub mod bubble;

pub trait Sort {
    fn items(&self) -> &Vec<f64>;
    fn step(&mut self) -> (&Vec<f64>, &SortPointer);
    fn is_active(&self) -> bool;
    fn activate_sort(&mut self);
    fn deactivate_sort(&mut self);
    fn get_pointer(&self) -> &SortPointer;
    fn get_name(&self) -> String;
}

pub struct SortPointer(pub usize, pub usize);
