pub trait Sort {
    fn items(&self) -> &Vec<f64>;
    fn step(&mut self) -> &Vec<f64>;
}

pub struct BubbleSort {
    pub input: Vec<f64>,
    pub step: u64,
    pub items: Vec<f64>,
    pub complete: bool,
}

impl BubbleSort {
    pub fn new(input: Vec<f64>) -> BubbleSort {
        let items = input.clone();
        BubbleSort {
            input,
            step: 0,
            items,
            complete: false,
        }
    }
}

impl Sort for BubbleSort {
    fn step(&mut self) -> &Vec<f64> {
        self.step = self.step + 1;
        for i in 0..self.items.len() - 1 {
            if self.items[i] > self.items[i + 1] {
                self.items.swap(i, i + 1);
            }
        }
        &self.items
    }
    fn items(&self) -> &Vec<f64> {
        &self.items
    }
}
