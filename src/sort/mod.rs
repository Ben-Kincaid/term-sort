pub mod bubble;
pub mod insertion;
pub mod selection;

pub trait Sort {
    fn items(&self) -> &Vec<f64>;
    fn step(&mut self) -> (&Vec<f64>, &SortPointer);
    fn is_active(&self) -> bool;
    fn is_sorted(&self) -> bool;
    fn toggle_sort(&mut self);
    fn activate_sort(&mut self);
    fn deactivate_sort(&mut self);
    fn get_pointer(&self) -> &SortPointer;
    fn get_name(&self) -> String;
    fn get_current_step(&self) -> usize;
    fn reset(&mut self, items: Vec<f64>);
}

pub struct SortPointer(pub usize, pub usize);

pub fn generate_random_data(size: usize) -> Vec<f64> {
    use rand::{distributions::Standard, Rng};

    rand::thread_rng()
        .sample_iter::<f64, Standard>(Standard)
        .take(size)
        .map(|x| x * 100.0)
        .collect()
}

#[cfg(test)]
pub mod test_util {
    pub fn is_sorted(data: &Vec<f64>) -> bool {
        data.windows(2).all(|w| w[0] <= w[1])
    }
}
