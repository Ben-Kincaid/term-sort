use crate::sort::{Sort, SortPointer};

pub struct BubbleSort {
    pub input: Vec<f64>,
    pub step: u64,
    pub items: Vec<f64>,
    pub complete: bool,
    pub active: bool,
    pub iterator: Box<dyn Iterator<Item = (Vec<f64>, SortPointer)>>,
    pub pointer: SortPointer,
}

impl BubbleSort {
    pub fn new(input: Vec<f64>) -> BubbleSort {
        let items = input.clone();
        let mut iterator_target = input.clone();
        BubbleSort {
            input,
            step: 0,
            items,
            complete: false,
            active: false,
            iterator: create_iterator(&mut iterator_target),
            pointer: SortPointer(0, 1),
        }
    }
}

impl Sort for BubbleSort {
    fn step(&mut self) -> (&Vec<f64>, &SortPointer) {
        if let Some((data, pointer)) = self.iterator.next() {
            self.items = data;
            self.pointer = pointer;
        } else {
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
        "Bubble Sort".to_string()
    }
}

fn create_iterator(input: &mut Vec<f64>) -> Box<dyn Iterator<Item = (Vec<f64>, SortPointer)>> {
    let mut result = vec![];
    for i in 0..input.len() - 1 {
        for j in 0..input.len() - 1 - i {
            if input[j] > input[j + 1] {
                input.swap(j, j + 1);
            }
            result.push((input.clone(), SortPointer(j, j + 1)));
        }
    }
    Box::new(result.into_iter())
}
