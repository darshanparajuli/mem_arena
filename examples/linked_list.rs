extern crate mem_arena;

use std::iter::{IntoIterator, Iterator};

use mem_arena::*;

fn main() {
    // Allocate 4 KB
    let mut arena = MemArena::alloc(1024 * 4);

    let mut list = LinkedList::new(&mut arena);
    for i in 0..12 {
        list.push(i);
    }

    for l in list.into_iter() {
        println!("{}", l);
    }
}

#[derive(Debug)]
enum NodeType<'a, T> {
    Empty,
    Node(&'a mut Node<'a, T>),
}

#[derive(Debug)]
struct Node<'a, T> {
    data: T,
    next: NodeType<'a, T>,
}

struct LinkedList<'a, T> {
    arena: &'a mut MemArena,
    root: NodeType<'a, T>,
}

impl<'a, T> LinkedList<'a, T> {
    fn new(arena: &'a mut MemArena) -> Self {
        Self {
            arena,
            root: NodeType::Empty,
        }
    }

    fn push(&mut self, data: T)
    where
        T: std::fmt::Debug,
    {
        let mut node = &mut self.root;
        loop {
            match node {
                NodeType::Empty => {
                    let n = self.arena.push::<Node<'a, T>>();
                    n.data = data;
                    *node = NodeType::Node(n);
                    break;
                }
                NodeType::Node(n) => {
                    node = &mut n.next;
                }
            }
        }
    }
}

struct LinkedListIntoIter<'a, 'b: 'a, T> {
    next: &'b NodeType<'a, T>,
}

impl<'a, 'b: 'a, T> Iterator for LinkedListIntoIter<'a, 'b, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.next {
            NodeType::Node(n) => {
                self.next = &n.next;
                Some(&n.data)
            }
            NodeType::Empty => None,
        }
    }
}

impl<'a, 'b: 'a, T> IntoIterator for &'b LinkedList<'a, T> {
    type Item = &'a T;
    type IntoIter = LinkedListIntoIter<'a, 'b, T>;

    fn into_iter(self) -> Self::IntoIter {
        let next = &self.root;
        LinkedListIntoIter { next }
    }
}
