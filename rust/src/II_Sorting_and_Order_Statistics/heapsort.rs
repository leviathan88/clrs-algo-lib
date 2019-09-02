// Chapter 6
// 6.1 Heaps
// 6.2 Maintaing the heap property
// 6.3 Building a heap
// 6.4 The heapsort algorithm

pub fn min_heapsort<T>(vec: Vec<T>) -> Vec<T> where T: PartialOrd + Copy {
    Heapsort::min_heapsort(vec)
}

pub fn max_heapsort<T>(vec: Vec<T>) -> Vec<T> where T: PartialOrd + Copy {
    Heapsort::max_heapsort(vec)
}

struct Heapsort<T: PartialOrd + Copy> {
    size: usize,
    heap: Vec<T>
}

impl<T> Heapsort<T> where T: PartialOrd + Copy {

    pub fn min_heapsort(vec: Vec<T>) -> Vec<T> {
        let mut hs = Heapsort::new(vec);
        hs.heap.reverse();
        hs.heap
    }

    pub fn max_heapsort(vec: Vec<T>) -> Vec<T> {
        Heapsort::new(vec).heap
    }


    fn new(vec: Vec<T>) -> Self {
        let mut hs = Heapsort {
            size: vec.len() - 1,
            heap: vec
        };

        hs.build_max_heap();
        hs.heapsort();
        hs
    }

    fn heapsort(&mut self) {
        for i in (2..=self.heap.len() - 1).rev()  {
            self.swap_two_elements(0, i);
            self.size -= 1;
            self.bubble_down(0);
        }
    }

    fn build_max_heap(&mut self) {
        let half = (self.heap.len() as f64 / 2 as f64 ).floor() as usize;

        for i in (0..=half).rev()  {
            self.bubble_down(i);
        }
        
    }

    /// Max-Heapify(A, i) function from book
    fn bubble_down(&mut self, index: usize) {
        let element = self.heap[index];
        let left_child_index = HelperSort::get_left_child_index(index);
        let right_child_index = HelperSort::get_right_child_index(index);

        if left_child_index <= self.size {
            let left_child_element = self.heap[left_child_index];

            if right_child_index <= self.size {
                let right_child_element = self.heap[right_child_index];

                if element > left_child_element && element > right_child_element {
                    let correct_index = if left_child_element < right_child_element { left_child_index } else { right_child_index };
                    self.swap_two_elements_with_bubble_down(index, correct_index);
                } else if element > right_child_element {
                    self.swap_two_elements_with_bubble_down(index, right_child_index);
                } else if element > left_child_element {
                    self.swap_two_elements_with_bubble_down(index, left_child_index);
                }
            } else {
                self.swap_two_elements_with_bubble_down(index, left_child_index);
            }
        }
    }

    fn swap_two_elements(&mut self, current_index: usize, new_index: usize) {
        let element = self.heap[current_index];
        let new_element = self.heap[new_index];
        self.heap[current_index] = new_element;
        self.heap[new_index] = element;
    }

    fn swap_two_elements_with_bubble_down(&mut self, current_index: usize, new_index: usize) {
        self.swap_two_elements(current_index, new_index);
        self.bubble_down(new_index);
    }
    
}

struct HelperSort {}

impl HelperSort {
    fn get_parent_index(i: usize) -> usize {
        ((i / 2) as f64).floor() as usize    
    }

    fn get_left_child_index(i: usize) -> usize {
        2 * i + 1
    }

    fn get_right_child_index(i: usize) -> usize {
        2 * i + 2
    }
}

mod tests {
    use super::*;

    fn get_unsorted_vec_of_i32() -> Vec<i32> {
        vec![7,4,6,9,1,2,3,4,5]
    }

    fn get_unsoreted_vec_of_f64() -> Vec<f64> {
        vec![7.12,4.4,6.09,9.17,1.12,2.01,3.0,4.42,5.12, 5.13, 1.09, 7.17]
    }

    #[test]
    fn test_min_heapsort_with_i32() {
        assert_eq!(min_heapsort(get_unsorted_vec_of_i32()), [1,2,3,4,4,5,6,7,9]);
    }

    #[test]
    fn test_max_heapsort_with_i32() {
        assert_eq!(max_heapsort(get_unsorted_vec_of_i32()), [9,7,6,5,4,4,3,2,1]);
    }

    #[test]
    fn test_min_heapsort_with_f64() {
        assert_eq!(min_heapsort(get_unsoreted_vec_of_f64()), [1.09, 1.12, 2.01, 3.0, 4.4, 4.42, 5.12, 5.13, 6.09, 7.12, 7.17, 9.17]);
    }

    #[test]
    fn test_max_heapsort_with_f64() {
        assert_eq!(max_heapsort(get_unsoreted_vec_of_f64()), [9.17, 7.17, 7.12, 6.09, 5.13, 5.12, 4.42, 4.4, 3.0, 2.01, 1.12, 1.09]);
    }
}