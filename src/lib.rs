#[derive(Debug)]
pub struct Heap {
    list: Vec<i32>
}

impl Heap {
    pub fn new() -> Heap {
        Heap { list: vec![] }
    }

    pub fn peek(&self) -> Option<i32> {
        if self.list.len() != 0 { Some(self.list[0]) } else { None }
    }

    pub fn push(&mut self, value: i32) {
        self.list.push(value);
        let mut index = self.list.len() - 1;

        while !self.is_valid(index) {
            let parent = self.parent(index);
            self.list[index] = self.list[parent];
            self.list[parent] = value;
            index = parent;
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
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

#[cfg(test)]
mod tests {
    use super::Heap;

    #[test]
    fn empty_returns_none() {
        let mut heap = Heap::new();

        assert_eq!(heap.peek(), None);
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn add_simple() {
        let mut heap = Heap::new();

        heap.push(1);
        heap.push(2);
        heap.push(3);

        assert_eq!(heap.list, vec![3, 1, 2]);
    }

    #[test]
    fn pop_simple() {
        let mut heap = Heap::new();

        heap.push(1);
        heap.push(2);
        heap.push(3);

        let pop1 = heap.pop();
        assert_eq!(pop1, Some(3));
        assert_eq!(heap.list, vec![2, 1]);

        let pop2 = heap.pop();
        assert_eq!(pop2, Some(2));
        assert_eq!(heap.list, vec![1]);

        let pop3 = heap.pop();
        assert_eq!(pop3, Some(1));
        assert_eq!(heap.list, vec![]);

        let pop4 = heap.pop();
        assert_eq!(pop4, None);
        assert_eq!(heap.list, vec![]);
    }
}