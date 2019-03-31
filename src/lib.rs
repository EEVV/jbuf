use std::vec;
use std::iter::IntoIterator;
use std::slice::Iter;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct JVec<T>(Vec<Option<T>>);

impl<T> JVec<T> {
    pub fn new() -> JVec<T> {
        JVec(vec![])
    }

    pub fn push(&mut self, i: Option<T>) {
        self.0.push(i);
    }

    pub fn remove(&mut self, i: usize) -> Option<T> {
        if self.0.len() > i {
            self.0.remove(i)
        } else {
            None
        }
    }

    // true if shifted
    pub fn insert(&mut self, i: usize, d: T) -> bool {
        let some = Some(d);

        if self.0.len() > i {
            self.0.insert(i, some);

            true
        } else {
            self[i] = some;

            false
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> Iter<Option<T>> {
        self.0.iter()
    } 
}

impl<T> IntoIterator for JVec<T> {
    type Item = Option<T>;
    type IntoIter = vec::IntoIter<Option<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> Index<usize> for JVec<T> {
    type Output = Option<T>;

    fn index(&self, i: usize) -> &Option<T> {
        if self.0.len() > i {
            &self.0[i]
        } else {
            &None
        }
    }
}

impl<T> IndexMut<usize> for JVec<T> {
    fn index_mut(&mut self, i: usize) -> &mut Option<T> {
        for _ in self.0.len()..=i {
            self.0.push(None);
        }

        &mut self.0[i]
    }
}
