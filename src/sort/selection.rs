use crate::sort::{Sort, SortPointer};

pub struct SelectionSort {
    pub input: Vec<f64>,
    pub items: Vec<f64>,
    pub step: usize,
    pub complete: bool,
    pub active: bool,
    pub iterator: Box<dyn Iterator<Item = (Vec<f64>, SortPointer)>>,
    pub pointer: SortPointer,
}

impl SelectionSort {
    pub fn new(input: Vec<f64>) -> SelectionSort {
        let items = input.clone();
        let mut iterator_target = input.clone();
        SelectionSort {
            input,
            items,
            step: 0,
            complete: false,
            active: false,
            iterator: create_iterator(&mut iterator_target),
            pointer: SortPointer(0, 1),
        }
    }
}

impl Sort for SelectionSort {
    fn step(&mut self) -> (&Vec<f64>, &SortPointer) {
        if let Some((data, pointer)) = self.iterator.next() {
            self.items = data;
            self.pointer = pointer;
            self.step = self.step + 1;
        } else {
            self.complete = true;
            self.deactivate_sort();
        }

        (&self.items, &self.pointer)
    }
    fn items(&self) -> &Vec<f64> {
        &self.items
    }
    fn is_active(&self) -> bool {
        self.active
    }
    fn is_sorted(&self) -> bool {
        self.complete
    }
    fn toggle_sort(&mut self) {
        self.active = !self.active;
    }
    fn activate_sort(&mut self) {
        self.active = true;
    }
    fn deactivate_sort(&mut self) {
        self.active = false;
    }
    fn get_pointer(&self) -> &SortPointer {
        &self.pointer
    }
    fn get_name(&self) -> String {
        "Selection Sort".to_string()
    }
    fn get_current_step(&self) -> usize {
        self.step
    }
    fn reset(&mut self, items: Vec<f64>) {
        self.input = items.clone();
        self.items = items.clone();
        self.iterator = create_iterator(&mut self.input);
        self.step = 0;
        self.complete = false;
        self.active = false;
    }
}

pub fn create_iterator(input: &mut Vec<f64>) -> Box<dyn Iterator<Item = (Vec<f64>, SortPointer)>> {
    let mut result = vec![];
    for i in 0..input.len() {
        let mut min_index = i;
        for j in i + 1..input.len() {
            if input[j] < input[min_index] {
                min_index = j;
            }
            result.push((input.clone(), SortPointer(i, j)));
        }
        if min_index != i {
            input.swap(i, min_index);
            result.push((input.clone(), SortPointer(i, min_index)));
        }
    }
    Box::new(result.into_iter())
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::sort::{generate_random_data, test_util};

    #[test]
    fn test_final_sort() {
        let items = generate_random_data(100);
        let result = create_iterator(&mut items.clone()).last();
        let (data, _) = result.unwrap();
        let sorted = test_util::is_sorted(&data);
        assert!(sorted);
    }
}
