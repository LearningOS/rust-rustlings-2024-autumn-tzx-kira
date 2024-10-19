/*
	heap
	This question requires you to implement a binary heap function
*/
// I AM NOT DONE

use std::cmp::Ord;
use std::default::Default;
use std::mem::replace;
pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            // items: Vec::new(),
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        // self.count += 1;  
        // self.items.push(value);  
        // let mut idx = self.count;  
  
        // while idx > 1 && (self.comparator)(&self.items[self.parent_idx(idx)], &self.items[idx]) {  
        //     // 将需要不可变借用的逻辑（如 parent_idx）和需要可变借用的逻辑（如 swap）分离到不同的方法中或不同的函数调用中。
        //     let parent = self.parent_idx(idx)-1; 
        //     self.items.swap(idx, parent);  
        //     idx = self.parent_idx(idx);  
        // }  

        // self.count += 1;  
        // self.items.push(value);  
        // self.sift_up(self.count); 

        // 将新元素添加到数组末尾
        self.items.push(value);
        self.count += 1;

        // 上浮调整
        let mut idx = self.count;
        while idx > 1 && (self.comparator)(&self.items[idx], &self.items[self.parent_idx(idx)]) {
            let mut parent= self.parent_idx(idx);
            self.items.swap(idx, parent);
            idx = self.parent_idx(idx);
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
        // (idx - 1) / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
        // 2 * (idx - 1) + 1 <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
        // 2 * (idx - 1) + 1
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
        // 2 * (idx - 1) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        // 0
		// let left = self.left_child_idx(idx);  
        // let right = self.right_child_idx(idx);  
  
        // if left > self.count {  
        //     return idx;  
        // }  
  
        // let left_value = &self.items[left];  
        // let mut smallest = left;  
  
        // if right <= self.count && (self.comparator)(&self.items[right], left_value) {  
        //     smallest = right;  
        // }  
  
        // smallest


        // let left = self.left_child_idx(idx);  
        // let right = self.right_child_idx(idx);  
  
        // if left > self.count {  
        //     return idx;  
        // }  
  
        // let smallest = if right > self.count || (self.comparator)(&self.items[left - 1], &self.items[right - 1]) {  
        //     left  
        // } else {  
        //     right  
        // };  
  
        // if (self.comparator)(&self.items[idx - 1], &self.items[smallest - 1]) {  
        //     idx  
        // } else {  
        //     smallest  
        // }

        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);

        // 如果没有右孩子，返回左孩子
        if right > self.count {
            return left;
        }

        // 返回较小的孩子
        if (self.comparator)(&self.items[left], &self.items[right]) {
            left
        } else {
            right
        }
    }

    // fn sift_up(&mut self,mut idx: usize) {  
    //     while idx > 1 && !(self.comparator)(&self.items[self.parent_idx(idx) - 1], &self.items[idx - 1]) {  
    //         let parent = self.parent_idx(idx) - 1;
    //         self.items.swap(parent, idx - 1);  
    //         idx = self.parent_idx(idx);  
    //     }  
    // }  
  
    // fn sift_down(&mut self,mut idx: usize) -> usize {  
    //     let start_idx = idx;  
    //     let mut child_idx = self.smallest_child_idx(idx);  
  
    //     while child_idx <= self.count && !(self.comparator)(&self.items[idx - 1], &self.items[child_idx - 1]) {  
    //         self.items.swap(idx - 1, child_idx - 1);  
    //         idx = child_idx;  
    //         child_idx = self.smallest_child_idx(idx);  
    //     }  
    //     start_idx  
    // }

    fn sift_down(&mut self, mut idx: usize) {
        while self.children_present(idx) {
            let smallest_child = self.smallest_child_idx(idx);
            if (self.comparator)(&self.items[smallest_child], &self.items[idx]) {
                self.items.swap(idx, smallest_child);
            } else {
                break;
            }
            idx = smallest_child;
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
		// None
        // if self.is_empty() {  
        //     return None;  
        // }  
  
        // let root = self.items[1].clone();  
        // self.items[1] = self.items.pop().unwrap_or_default();  
        // self.count -= 1;  
  
        // let mut idx = 1;  
        // while self.children_present(idx) {  
        //     let smallest_child = self.smallest_child_idx(idx);  
        //     if !(self.comparator)(&self.items[smallest_child], &self.items[idx]) {  
        //         break;  
        //     }  
        //     self.items.swap(idx, smallest_child);  
        //     idx = smallest_child;  
        // }  
  
        // Some(root)
        // if self.is_empty() {  
        //     return None;  
        // }  
        // // Move the root to the end and reduce count  
        // let tmp=self.items.pop().unwrap_or_default();
        // let result = replace(&mut self.items[0], tmp);  
        // self.count -= 1;  
        // // Fix the heap  
        // self.sift_down(1);  
        // Some(result)

        if self.is_empty() {
            return None;
        }

        // 取出堆顶元素（第1个元素）
        let result = self.items.swap_remove(1);
        self.count -= 1;

        // 如果堆不为空，调整堆
        if !self.is_empty() {
            self.sift_down(1);
        }

        Some(result)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}