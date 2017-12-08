use std::cmp::max;

fn main() {
    let mut heap = Heap::new();
    println!("{:?}", heap);

    heap.push(1);
    println!("{:?}", heap);
    heap.push(2);
    println!("{:?}", heap);
    heap.push(3);
    println!("{:?}", heap);

    println!("{:?}", heap.pop());
    println!("{:?}", heap);
    println!("{:?}", heap.pop());
    println!("{:?}", heap);
    println!("{:?}", heap.pop());
    println!("{:?}", heap);
}

#[derive(Debug)]
struct Heap {
    list: Vec<i32>
}

impl Heap {
    fn new() -> Heap {
        Heap { list: vec![] }
    }

    fn peek(&self) -> Option<i32> {
        if self.list.len() != 0 { Some(self.list[0]) } else { None }
    }

    fn push(&mut self, value: i32) {
        self.list.push(value);
        let mut index = self.list.len() - 1;

        while !self.is_valid(index) {
            let parent = self.parent(index);
            self.list[index] = self.list[parent];
            self.list[parent] = value;
            index = parent;
        }
    }

    fn pop(&mut self) -> Option<i32> {
        let result = match self.peek() {
            Some(elem) => elem,
            None => return None
        };

        if self.list.len() > 1 {
            self.list[0] = self.list.pop().unwrap();
        }
        else {
            self.list.remove(0);
        }

        let mut index = 0;

        loop {
            let (left_index, right_index) = (self.left(index), self.right(index));
            let mut largest = index;

            // Check if the left child is larger
            if left_index < self.list.len() && self.list[left_index] > self.list[largest] {
                largest = left_index;
            }
            // Check if the right child is larger
            if right_index < self.list.len() && self.list[right_index] > self.list[largest] {
                largest = right_index;
            }

            // Stop once the tree is valid
            if largest == index {
                break;
            }

            // Otherwise swap them and try again
            self.list.swap(index, largest);
            index = largest;
        }

        Some(result)
    }

    fn is_valid(&self, index: usize) -> bool {
        index == 0 || self.list[self.parent(index)] >= self.list[index]
    }

    fn parent(&self, index: usize) -> usize {
        (index - 1) / 2 as usize
    }

    fn left(&self, index: usize) -> usize {
        index * 2 + 1
    }

    fn right(&self, index: usize) -> usize {
        self.left(index) + 1
    }
}