use std::ops::{Index, IndexMut};

#[derive(Clone)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}
#[derive(Clone)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T: std::fmt::Debug> std::fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}, {:?}", self.data, self.next)
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "LinkedList {:?}", self.head)
    }
}

impl<T> Index<usize> for LinkedList<T>  {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let mut current = &self.head;
        let mut i = self.size;
        while let Some(node) = current {
            if i < index { break; }
            if i == index { return &node.data; }
            current = &node.next;
            i -= 1;
        }
        panic!("Index out of bounds Max is {}", i);
    }
}

impl<T> IndexMut<usize> for LinkedList<T>  {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let mut current = &mut self.head;
        let mut i = self.size;
        while let Some(node) = current {
            if i < index { break; }
            if i == index { return &mut node.data; }
            current = &mut node.next;
            i -= 1;
        }
        panic!("Index out of bounds Max is {}", i);
    }
}

impl<T: std::fmt::Debug + 'static + PartialEq + Ord + Copy> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None, size: 0 }
    }

    pub fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn get_size(&self) -> &usize {
        &self.size
    }

    pub fn get_element_type(&self) -> &str {
        std::any::type_name::<Self>()
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.data
        })
    }

    pub fn clear(&mut self) {
        self.head = None;
        self.size = 0;
    }

    pub fn iter(&self) -> LinkedListIter<T> {
        LinkedListIter {
            current: &self.head,
        }
    }

    pub fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = LinkedList::new();
        for item in iter {
            list.push(item);
        }
        list
    }

    pub fn exists(&self, data: T) -> bool {
        let mut current = &self.head;
        while let Some(node) = current {
            if node.data == data {
                return true;
            }
            current = &node.next;
        }
        false
    }

    pub fn get(&self, data: T) -> Option<&T> {
        let mut current = &self.head;
        let mut return_data = None;
        while let Some(node) = current {
            if node.data == data {
                return_data = Some(&node.data);
            }
            current = &node.next;
        }
        return_data
    }

    pub fn is_empty(&self) -> bool {
        self.get_size() == &0
    }



    pub fn debug_memory(&self) {
        println!("LinkedList Debug:");
        println!("{:#?}", self);
    }

    pub fn print_memory_layout(&self) {
        let mut current = &self.head;
        let size = self.get_size() - 1;
        println!("LinkedList Memory Layout (FIFO):");
        println!("-----------------------------------");
        println!("{:<4} |\t{:<4}\t| {:<17}", "Node", "Data", "Memory Location");

        let mut i = 0;
        while let Some(node) = current {
            let raw_ptr: *const Node<T> = &**node;
            if i == 0{ println!("{:<4} |\t{:<4?}\t| {:<17p} <----- Last Push", i, &node.data, raw_ptr); }
            else if i == size{ println!("{:<4} |\t{:<4?}\t| {:<17p} <----- First Push", i, &node.data, raw_ptr); }
            else { println!("{:<4} |\t{:<4?}\t| {:<17p}", i, &node.data, raw_ptr) }
            current = &node.next;
            i += 1;
        }
        println!("-----------------------------------");
    }

    pub fn sorting(&mut self) {
        let mut vec: Vec<T> = self.iter().map(|x| *x).collect();
        quick_sort(&mut vec, 0, self.get_size() - 1);
        self.head = LinkedList::from_iter(vec).head;
    }

}

fn quick_sort<T: Ord + Copy>(items: &mut [T], fst: usize, lst: usize) {
    if fst >= lst {
        return;
    }
    let mut i = fst;
    let mut j = lst;
    let x = items[(fst + lst) / 2];
    while i < j {
        while items[i] < x {
            i += 1;
        }
        while items[j] > x {
            j -= 1;
        }
        if i <= j {
            items.swap(i, j);
            i += 1;
            if j > 0 {
                j -= 1;
            }
        }
    }
    quick_sort(items, fst, j);
    quick_sort(items, i, lst);
}

pub struct LinkedListIter<'a, T> {
    current: &'a Option<Box<Node<T>>>,
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.as_ref().map(|node| {
            self.current = &node.next;
            &node.data
        })
    }
}