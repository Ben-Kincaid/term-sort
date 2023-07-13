use crate::sort::{Sort, SortPointer};

pub struct InsertionSort {
    pub input: Vec<f64>,
    pub items: Vec<f64>,
    pub complete: bool,
    pub active: bool,
    pub iterator: Box<dyn Iterator<Item = (Vec<f64>, SortPointer)>>,
    pub pointer: SortPointer,
}

impl InsertionSort {
    pub fn new(input: Vec<f64>) -> InsertionSort {
        let items = input.clone();
        let mut iterator_target = input.clone();
        InsertionSort {
            input,
            items,
            complete: false,
            active: false,
            iterator: create_iterator(&mut iterator_target),
            pointer: SortPointer(0, 1),
        }
    }
}

impl Sort for InsertionSort {
    fn step(&mut self) -> (&Vec<f64>, &SortPointer) {
        if let Some((data, pointer)) = self.iterator.next() {
            self.items = data;
            self.pointer = pointer;
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
        "Insertion Sort".to_string()
    }
}

pub fn create_iterator(input: &mut Vec<f64>) -> Box<dyn Iterator<Item = (Vec<f64>, SortPointer)>> {
    let mut result = vec![];
    for i in 1..input.len() {
        let mut j = i;
        while j > 0 && input[j] < input[j - 1] {
            input.swap(j, j - 1);
            j = j - 1;
            result.push((input.clone(), SortPointer(i, j)));
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
