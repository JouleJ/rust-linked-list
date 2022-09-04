use std::ops::Index;
use std::sync::Arc;
pub use std::iter::FromIterator;

enum Node<T> {
    Nil,
    Cons(T, Arc<Node<T>>)
}

fn index_impl<T>(node: &Node<T>, position: usize) -> Option<&T> {
    if position == 0 {
        match &node {
            &Node::Nil => None,
            &Node::Cons(element, _) => Some(&element)
        }
    } else {
        match &node {
            &Node::Nil => None,
            &Node::Cons(_, next) => index_impl(&next, position - 1)
        }
    }
}

fn length_impl<T>(node: &Node<T>) -> usize {
    match &node {
        &Node::Nil => 0,
        &Node::Cons(_, next) => 1 + length_impl(&next)
    }
}

fn from_iter_impl<T>(mut iter: impl Iterator<Item=T>) -> Arc<Node<T>> {
    match iter.next() {
        None => Arc::new(Node::Nil),
        Some(element) => Arc::new(Node::Cons(element, from_iter_impl(iter)))
    }
}

fn from_iter_ref_impl<'a, T: 'a + Clone>(mut iter: impl Iterator<Item=&'a T>) -> Arc<Node<T>> {
    match &iter.next() {
        &None => Arc::new(Node::Nil),
        &Some(element) => Arc::new(Node::Cons(element.clone(), from_iter_ref_impl(iter)))
    }
}

pub struct List<T> {
    head: Arc<Node<T>>
}

pub struct NodeIterator<'a, T> {
    current: &'a Node<T>
}

impl<'a, T> Iterator for NodeIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let &Node::Cons(element, next) = &self.current {
            self.current = next;
            Some(&element)
        } else {
            None
        }
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head : Arc::new(Node::Nil) }
    }

    pub fn push_front(&self, element: T) -> Self {
        List {
            head : Arc::new(Node::Cons(element, self.head.clone()))
        }
    }

    pub fn from_slice(slice: &[T]) -> Self
        where T: Clone {
        
        List {
            head : from_iter_ref_impl(slice.into_iter())
        }
    }

    pub fn get(&self, position: usize) -> Option<&T> {
        index_impl(&self.head, position)
    }

    pub fn len(&self) -> usize {
        length_impl(&self.head)
    }

    pub fn iter(&self) -> NodeIterator<T> {
        NodeIterator { current: &*self.head }
    }

    pub fn head(&self) -> Option<&T> {
        match &*self.head {
            Node::Nil => None,
            Node::Cons(element, _) => Some(&element)
        }
    }

    pub fn tail(&self) -> Option<Self> {
        match &*self.head {
            Node::Nil => None,
            Node::Cons(_, tail) => Some(List{head : tail.clone()})
        }
    }
}

impl<T> Index<usize> for List<T> {
    type Output = T;

    fn index(&self, position: usize) -> &Self::Output {
        index_impl(&self.head, position).unwrap()
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        List { head : from_iter_impl(iter.into_iter()) }
    }
}

impl<'a, T: 'a + Clone> FromIterator<&'a T> for List<T> {
    fn from_iter<I: IntoIterator<Item=&'a T>>(iter: I) -> Self {
        List { head : from_iter_ref_impl(iter.into_iter()) }
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = NodeIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
