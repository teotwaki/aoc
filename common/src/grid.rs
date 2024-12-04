use crate::Coordinates;
use num_traits::Num;
use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Clone, Default)]
pub struct Grid<T, U> {
    items: HashMap<Coordinates<U>, T>,
    min_x: U,
    max_x: U,
    min_y: U,
    max_y: U,
}

impl<T, U> Grid<T, U>
where
    T: Clone + Default,
    U: Eq + Hash + Default + Copy + PartialOrd + Num,
{
    #[inline]
    pub fn new() -> Self {
        Grid {
            items: HashMap::new(),
            ..Default::default()
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Grid {
            items: HashMap::with_capacity(capacity),
            ..Default::default()
        }
    }

    fn store_min_max(&mut self, pos: Coordinates<U>) {
        if self.min_x > pos.x() {
            self.min_x = pos.x();
        } else if self.max_x < pos.x() {
            self.max_x = pos.x();
        }

        if self.min_y > pos.y() {
            self.min_y = pos.y();
        } else if self.max_y < pos.y() {
            self.max_y = pos.y();
        }
    }

    pub fn store(&mut self, pos: Coordinates<U>, value: T) {
        self.items.insert(pos, value);
        self.store_min_max(pos);
    }

    pub fn get(&self, pos: &Coordinates<U>) -> Option<&T> {
        self.items.get(pos)
    }

    pub fn contains(&self, pos: &Coordinates<U>) -> bool {
        self.get(pos).is_some()
    }

    #[inline]
    pub fn width(&self) -> U {
        self.max_x - self.min_x + U::one()
    }

    #[inline]
    pub fn height(&self) -> U {
        self.max_y - self.min_y + U::one()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&Coordinates<U>, &T)> {
        self.items.iter()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<T, U> IntoIterator for Grid<T, U> {
    type Item = <HashMap<Coordinates<U>, T> as IntoIterator>::Item;
    type IntoIter = <HashMap<Coordinates<U>, T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
