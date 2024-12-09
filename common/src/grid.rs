use crate::Coordinates;
use num_traits::{Num, PrimInt};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    ops::{AddAssign, SubAssign},
};

#[derive(Debug, Clone, Default)]
pub struct Grid<T, U> {
    items: HashMap<Coordinates<T>, U>,
    min_x: T,
    max_x: T,
    min_y: T,
    max_y: T,
}

impl<T, U> Grid<T, U>
where
    T: Eq + Hash + Default + Copy + PartialOrd + Num,
    U: Clone + Default,
{
    #[inline]
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            ..Default::default()
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: HashMap::with_capacity(capacity),
            ..Default::default()
        }
    }

    fn store_min_max(&mut self, pos: Coordinates<T>) {
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

    pub fn store(&mut self, pos: Coordinates<T>, value: U) {
        self.items.insert(pos, value);
        self.store_min_max(pos);
    }

    pub fn get(&self, pos: &Coordinates<T>) -> Option<&U> {
        self.items.get(pos)
    }

    pub fn get_mut(&mut self, pos: Coordinates<T>) -> &mut U {
        self.items.entry(pos).or_default()
    }

    pub fn contains(&self, pos: &Coordinates<T>) -> bool {
        self.get(pos).is_some()
    }

    #[inline]
    pub fn width(&self) -> T {
        self.max_x - self.min_x + T::one()
    }

    #[inline]
    pub fn height(&self) -> T {
        self.max_y - self.min_y + T::one()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&Coordinates<T>, &U)> {
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

    #[inline]
    pub fn within_bounds(&self, pos: Coordinates<T>) -> bool {
        pos.x() >= self.min_x
            && pos.x() <= self.max_x
            && pos.y() >= self.min_y
            && pos.y() <= self.max_y
    }

    #[inline]
    pub fn remove(&mut self, pos: &Coordinates<T>) {
        self.items.remove(pos);
    }
}

impl<T, U> Grid<T, U>
where
    T: Eq + Hash + Default + Copy + PartialOrd + PrimInt + SubAssign + AddAssign,
    U: Clone + Default,
{
    pub fn bfs<
        E: Fn(Coordinates<T>, &U) -> bool,
        P: Fn((Coordinates<T>, &U), (Coordinates<T>, &U)) -> bool,
    >(
        &self,
        root: Coordinates<T>,
        is_target: E,
        pred: P,
    ) -> Vec<Coordinates<T>> {
        let mut q = VecDeque::from([root]);
        let mut explored = HashSet::from([root]);
        let mut parents = HashMap::new();

        while let Some(v) = q.pop_front() {
            if is_target(v, self.items.get(&v).unwrap()) {
                let mut path = vec![v];
                while let Some(&parent) = parents.get(path.last().unwrap()) {
                    path.push(parent);
                }

                path.reverse();

                return path;
            } else {
                for neighbor in v.neighbors() {
                    if self.within_bounds(neighbor)
                        && !explored.contains(&neighbor)
                        && pred(
                            (v, self.get(&v).unwrap()),
                            (neighbor, self.get(&neighbor).unwrap()),
                        )
                    {
                        parents.insert(neighbor, v);
                        explored.insert(neighbor);
                        q.push_back(neighbor);
                    }
                }
            }
        }

        vec![]
    }

    pub fn bfs_all<
        E: Fn(Coordinates<T>, &U) -> bool,
        P: Fn((Coordinates<T>, &U), (Coordinates<T>, &U)) -> bool,
    >(
        &self,
        root: Coordinates<T>,
        is_target: E,
        pred: P,
    ) -> Vec<Vec<Coordinates<T>>> {
        let mut q = VecDeque::from([vec![root]]);
        let mut explored = HashSet::from([root]);
        let ends = self
            .items
            .iter()
            .filter(|&(pos, v)| is_target(*pos, v))
            .map(|(&pos, _)| pos)
            .collect::<HashSet<_>>();

        let mut paths = vec![];

        while let Some(path) = q.pop_front() {
            let front = *path.last().unwrap();

            if ends.contains(&front) {
                paths.push(path);
            } else {
                for neighbor in front.neighbors() {
                    if self.within_bounds(neighbor)
                        && !explored.contains(&neighbor)
                        && pred(
                            (front, self.get(&front).unwrap()),
                            (neighbor, self.get(&neighbor).unwrap()),
                        )
                    {
                        explored.insert(neighbor);
                        let mut path = path.clone();
                        path.push(neighbor);
                        q.push_back(path);
                    }
                }
            }
        }

        paths
    }

    pub fn bfs_exhaustive<
        E: Fn(Coordinates<T>, &U) -> bool,
        P: Fn((Coordinates<T>, &U), (Coordinates<T>, &U)) -> bool,
    >(
        &self,
        root: Coordinates<T>,
        is_target: E,
        pred: P,
    ) -> Vec<Vec<Coordinates<T>>> {
        let mut q = VecDeque::from([vec![root]]);
        let ends = self
            .items
            .iter()
            .filter(|&(pos, v)| is_target(*pos, v))
            .map(|(&pos, _)| pos)
            .collect::<HashSet<_>>();

        let mut paths = vec![];

        while let Some(path) = q.pop_front() {
            let front = *path.last().unwrap();

            if ends.contains(&front) {
                paths.push(path);
            } else {
                for neighbor in front.neighbors() {
                    if self.within_bounds(neighbor)
                        && pred(
                            (front, self.get(&front).unwrap()),
                            (neighbor, self.get(&neighbor).unwrap()),
                        )
                    {
                        let mut path = path.clone();
                        path.push(neighbor);
                        q.push_back(path);
                    }
                }
            }
        }

        paths
    }
}

#[derive(Debug, Clone, Default)]
pub struct BooleanGrid<T> {
    items: HashSet<Coordinates<T>>,
    min_x: T,
    max_x: T,
    min_y: T,
    max_y: T,
}

impl<T> BooleanGrid<T>
where
    T: Eq + Hash + Default + Copy + PartialOrd + Num,
{
    #[inline]
    pub fn new() -> Self {
        Self {
            items: HashSet::new(),
            ..Default::default()
        }
    }

    pub fn mark(&mut self, pos: Coordinates<T>) {
        self.items.insert(pos);
        self.store_min_max(pos);
    }

    fn store_min_max(&mut self, pos: Coordinates<T>) {
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

    pub fn contains(&self, pos: &Coordinates<T>) -> bool {
        self.items.contains(pos)
    }

    #[inline]
    pub fn width(&self) -> T {
        self.max_x - self.min_x + T::one()
    }

    #[inline]
    pub fn height(&self) -> T {
        self.max_y - self.min_y + T::one()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &Coordinates<T>> {
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

    #[inline]
    pub fn within_bounds(&self, pos: Coordinates<T>) -> bool {
        pos.x() >= self.min_x
            && pos.x() <= self.max_x
            && pos.y() >= self.min_y
            && pos.y() <= self.max_y
    }

    #[inline]
    pub fn remove(&mut self, pos: &Coordinates<T>) {
        self.items.remove(pos);
    }
}

#[derive(Debug, Clone, Default)]
pub struct BoundedGrid<T, U> {
    grid: Grid<T, U>,
    min: Coordinates<T>,
    max: Coordinates<T>,
}

impl<T, U> BoundedGrid<T, U>
where
    T: Eq + Hash + Default + Copy + PartialOrd + Num,
    U: Clone + Default,
{
    #[inline]
    pub fn new(min: Coordinates<T>, max: Coordinates<T>) -> Self {
        Self {
            grid: Grid::new(),
            min,
            max,
        }
    }

    pub fn store(&mut self, pos: Coordinates<T>, value: U) -> bool {
        if self.within_bounds(pos) {
            self.grid.store(pos, value);
            true
        } else {
            false
        }
    }

    pub fn get(&self, pos: &Coordinates<T>) -> Option<&U> {
        self.grid.get(pos)
    }

    pub fn get_mut(&mut self, pos: Coordinates<T>) -> &mut U {
        self.grid.get_mut(pos)
    }

    pub fn contains(&self, pos: &Coordinates<T>) -> bool {
        self.grid.get(pos).is_some()
    }

    #[inline]
    pub fn width(&self) -> T {
        self.max.x() - self.min.x() + T::one()
    }

    #[inline]
    pub fn height(&self) -> T {
        self.max.y() - self.min.y() + T::one()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&Coordinates<T>, &U)> {
        self.grid.iter()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.grid.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.grid.is_empty()
    }

    #[inline]
    pub fn within_bounds(&self, pos: Coordinates<T>) -> bool {
        pos.x() >= self.min.x()
            && pos.x() <= self.max.x()
            && pos.y() >= self.min.y()
            && pos.y() <= self.max.y()
    }

    #[inline]
    pub fn remove(&mut self, pos: &Coordinates<T>) {
        self.grid.remove(pos);
    }
}

#[derive(Debug, Clone, Default)]
pub struct BooleanBoundedGrid<T> {
    grid: BooleanGrid<T>,
    min: Coordinates<T>,
    max: Coordinates<T>,
}

impl<T> BooleanBoundedGrid<T>
where
    T: Eq + Hash + Default + Copy + PartialOrd + Num,
{
    #[inline]
    pub fn new(min: Coordinates<T>, max: Coordinates<T>) -> Self {
        Self {
            grid: BooleanGrid::new(),
            min,
            max,
        }
    }

    pub fn mark(&mut self, pos: Coordinates<T>) -> bool {
        if self.within_bounds(pos) {
            self.grid.mark(pos);
            true
        } else {
            false
        }
    }

    pub fn contains(&self, pos: &Coordinates<T>) -> bool {
        self.grid.contains(pos)
    }

    #[inline]
    pub fn width(&self) -> T {
        self.max.x() - self.min.x() + T::one()
    }

    #[inline]
    pub fn height(&self) -> T {
        self.max.y() - self.min.y() + T::one()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &Coordinates<T>> {
        self.grid.iter()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.grid.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.grid.is_empty()
    }

    #[inline]
    pub fn within_bounds(&self, pos: Coordinates<T>) -> bool {
        pos.x() >= self.min.x()
            && pos.x() <= self.max.x()
            && pos.y() >= self.min.y()
            && pos.y() <= self.max.y()
    }

    #[inline]
    pub fn remove(&mut self, pos: &Coordinates<T>) {
        self.grid.remove(pos);
    }
}
