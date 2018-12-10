#[derive(Debug)]
pub struct Heap<T: PartialOrd> {
    list: Vec<T>,
}

impl<T: PartialOrd> Heap<T> {
    pub fn new() -> Heap<T> {
        Heap { list: Vec::new() }
    }

    /// Get the maximum value in the heap or `None` if the heap is empty.
    pub fn peek(&self) -> Option<&T> {
        self.list.get(0)
    }

    /// Add a value to the heap.
    pub fn push(&mut self, value: T) {
        self.list.push(value);
        let index = self.list.len() - 1;
        self.heapify_up(index)
    }

    /// Remove the maximum value in the heap or `None` if the heap is empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.list.len() == 0 {
            return None;
        }

        // Get the maximum value
        let result = self.list.remove(0);

        // Move the last element to the front.
        // If there is only one element, the list is now empty.
        if self.list.len() > 0 {
            let last = self.list.pop().unwrap();
            self.list.insert(0, last);
        }

        self.heapify_down(0);

        Some(result)
    }

    /// Reorganize the heap upwards to make it valid again.
    /// The index parameter is where to start the heapify operation.
    fn heapify_up(&mut self, mut index: usize) {
        while !self.is_valid(index) {
            let parent = self.parent(index);
            self.list.swap(index, parent);
            index = parent;
        }
    }

    /// Reorganize the heap downwards to make it valid again.
    /// The index parameter is where to start the heapify operation.
    fn heapify_down(&mut self, mut index: usize) {
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
    }

    /// Check if the entry is valid (compares with parent)
    fn is_valid(&self, index: usize) -> bool {
        index == 0 || self.list[self.parent(index)] >= self.list[index]
    }

    /// Get the parent index of the entry at the index
    fn parent(&self, index: usize) -> usize {
        (index - 1) / 2 as usize
    }

    /// Get the left child index of the entry at the index
    fn left(&self, index: usize) -> usize {
        index * 2 + 1
    }

    /// Get the right child index of the entry at the index
    fn right(&self, index: usize) -> usize {
        self.left(index) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::Heap;

    #[test]
    fn empty_returns_none() {
        let mut heap: Heap<()> = Heap::new();

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
