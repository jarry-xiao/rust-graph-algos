use std::fmt::Debug;

pub struct PriorityQueue<T> {
    data: Vec<T>,
    size: usize,
    maxsize: Option<usize>,
}

pub fn build_priority_queue<T>(data: Option<Vec<T>>, maxsize: Option<usize>) -> PriorityQueue<T> {
    match data {
        Some(x) => 
            return PriorityQueue{
                data: x,
                size: 0,
                maxsize,
            },
        None =>
            return PriorityQueue{
                data: vec![],
                size: 0,
                maxsize,
            },
    };
}

impl<T> PriorityQueue<T> 
    where T: Copy + PartialOrd + Debug {

    pub fn print_data(&self) {
        println!("{:?} \n", self.data);
    }

    pub fn push(&mut self, val: T) {
        if self.maxsize.is_some() && self.size == self.maxsize.unwrap() {
            self.pop();
        }
        self.data.push(val);
        self._heapify_up(self.size);
        self.size += 1;
    }

    fn _heapify_up(&mut self, i: usize) {
        if i == 0 {
            return;
        }
        let parent = (i - 1) / 2; 
        if self.data[parent] > self.data[i] {
            self.data.swap(parent, i);
            self._heapify_up(parent);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        let top = self.data[0];
        self.size -= 1;
        self.data.swap(0, self.size);
        self._heapify_down(0);
        return Some(top);
    }

    fn _heapify_down(&mut self, root: usize) {
        let left = 2 * root + 1;
        let right = 2 * root + 2;
        let mut min = root;
        if left < self.size && self.data[left] > self.data[min] {
            min = left; 
        }
        if right < self.size && self.data[right] > self.data[min] {
            min = right; 
        }
        if min != root {
            self.data.swap(root, min);
            self._heapify_down(min);
        }
    }
}
