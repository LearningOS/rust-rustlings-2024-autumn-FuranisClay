/*
	heap
	This question requires you to implement a binary heap function
*/
//

//压扁的完全二叉树，二叉堆
// 1. 完全二叉树的性质
// 完全二叉树是指树的所有层都被填满，除了最后一层，最后一层的节点尽量向左靠拢。
// 由于完全二叉树的特性，使用数组存储时，可以避免使用指针或引用，使得内存的使用更加高效。
// 2. 数组表示
// 在数组中，二叉堆的节点存储在顺序位置，通常从索引 1 开始。根节点放在索引 1，其左子节点和右子节点则分别放在索引 2 和 3。从一个节点的位置 i 开始，子节点和父节点的位置关系如下：
// 父节点的索引：parent(i) = i / 2
// 左子节点的索引：left(i) = 2 * i
// 右子节点的索引：right(i) = 2 * i + 1

use std::cmp::Ord;
use std::default::Default;

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
        // 添加新元素到末尾
        self.items.push(value);
        self.count += 1;
        // 上滤操作
        self.swim(self.count);
    }

    fn swim(&mut self, mut idx: usize) {
        while idx > 1 {
            let parent_idx = self.parent_idx(idx); // 先提取父节点索引
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                self.items.swap(idx, parent_idx); // 只进行可变借用
                idx = parent_idx; // 继续向上移动
            } else {
                break; // 如果不需要交换，结束循环
            }
        }
    }
    

    fn sink(&mut self, mut idx: usize) {
        // 下滤过程：如果当前节点比其较小的子节点更大/小，则交换
        while self.children_present(idx) {
            let smallest_child = self.smallest_child_idx(idx);
            if (self.comparator)(&self.items[smallest_child], &self.items[idx]) {
                self.items.swap(idx, smallest_child);
            }
            idx = smallest_child;
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);
        
        if right > self.count {
            left
        } else if (self.comparator)(&self.items[left], &self.items[right]) {
            left
        } else {
            right
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
        if self.is_empty() {
            None
        } else {
            // 将堆顶元素移出
            let result = self.items[1].clone();
            self.items[1] = self.items[self.count].clone();
            self.items.pop();
            self.count -= 1;
            // 下滤操作
            self.sink(1);
            Some(result)
        }
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