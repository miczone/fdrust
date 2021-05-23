use core::ops::Range;
use core::iter::FromIterator;
use core::fmt::{Debug, Formatter, Result as FmtResult};
use core::slice::Iter;
use core::cmp;
#[cfg(feature = "std")]
use std::vec::{Vec, IntoIter};
#[cfg(not(feature = "std"))]
use alloc::vec::{Vec, IntoIter};
use smallvec::SmallVec;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct Element<K, V> {
    pub range: Range<K>,
    pub value: V,
}

impl<K, V> From<(Range<K>, V)> for Element<K, V> {
    fn from(tup: (Range<K>, V)) -> Element<K, V> {
        let (range, value) = tup;
        Element { range, value }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Hash)]
struct Node<K, V>{
    element: Element<K, V>,
    max: K,
}

#[derive(Clone, Debug, Deserialize, Serialize, Hash)]
pub struct IntervalTree<K, V> {
    data: Vec<Node<K, V>>,
}

impl<K: Ord + Clone, V, I: Into<Element<K, V>>> FromIterator<I> for IntervalTree<K, V> {
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        let mut nodes: Vec<_> = iter.into_iter().map(|i| i.into())
            .map(|element| Node { max: element.range.end.clone(), element }).collect();

        nodes.sort_unstable_by(|a, b| a.element.range.start.cmp(&b.element.range.start));

        if !nodes.is_empty() {
            Self::update_max(&mut nodes);
        }

        IntervalTree { data: nodes }
    }
}

#[derive(Debug)]
pub struct TreeIter<'a, K: 'a, V: 'a>(Iter<'a, Node<K, V>>);

impl<'a, K: 'a, V: 'a> Iterator for TreeIter<'a, K, V> {
    type Item = &'a Element<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| &x.element)
    }
}

impl<'a, K: 'a + Ord, V: 'a> IntoIterator for &'a IntervalTree<K, V> {
    type Item = &'a Element<K, V>;
    type IntoIter = TreeIter<'a, K, V>;

    fn into_iter(self) -> TreeIter<'a, K, V> {
        self.iter()
    }
}

#[derive(Debug)]
pub struct TreeIntoIter<K, V>(IntoIter<Node<K, V>>);

impl<K, V> IntoIterator for IntervalTree<K, V> {
    type Item = Element<K, V>;
    type IntoIter = TreeIntoIter<K, V>;

    fn into_iter(self) -> TreeIntoIter<K, V> {
        TreeIntoIter(self.data.into_iter())
    }
}

impl<K, V> Iterator for TreeIntoIter<K, V> {
    type Item = Element<K, V>;

    fn next(&mut self) -> Option<Element<K, V>> {
        self.0.next().map(|x| x.element)
    }
}

impl<K: Ord + Clone, V> IntervalTree<K, V> {
    fn update_max(nodes: &mut [Node<K, V>]) -> K {
        assert!(!nodes.is_empty());
        let i = nodes.len() / 2;
        if nodes.len() > 1 {
            {
                let (left, rest) = nodes.split_at_mut(i);
                if !left.is_empty() {
                    rest[0].max = cmp::max(rest[0].max.clone(), Self::update_max(left));
                }
            }

            {
                let (rest, right) = nodes.split_at_mut(i + 1);
                if !right.is_empty() {
                    rest[i].max = cmp::max(rest[i].max.clone(), Self::update_max(right));
                }
            }
        }

        nodes[i].max.clone()
    }
}

impl<K: Ord, V> IntervalTree<K, V> {
    fn items(&self) -> ItemVec {
        let mut items = SmallVec::new();
        if !self.data.is_empty() {
            items.push((0, self.data.len()));
        }
        items
    }

    pub fn query(&self, range: Range<K>) -> QueryIter<K, V> {
        QueryIter {
            items: self.items(),
            tree: self,
            query: Query::Range(range),
        }
    }

    pub fn query_point(&self, point: K) -> QueryIter<K, V> {
        QueryIter {
            items: self.items(),
            tree: self,
            query: Query::Point(point),
        }
    }

    pub fn iter(&self) -> TreeIter<K, V> {
        TreeIter(self.data.iter())
    }

    pub fn iter_sorted(&self) -> impl Iterator<Item=&Element<K, V>> {
        TreeIter(self.data.iter())
    }
}

#[derive(Clone, Debug)]
enum Query<K> {
    Point(K),
    Range(Range<K>),
}

impl<K: Ord> Query<K> {
    fn point(&self) -> &K {
        match *self {
            Query::Point(ref k) => k,
            Query::Range(ref r) => &r.start,
        }
    }

    fn go_right(&self, start: &K) -> bool {
        match *self {
            Query::Point(ref k) => k >= start,
            Query::Range(ref r) => &r.end > start,
        }
    }

    fn intersect(&self, range: &Range<K>) -> bool {
        match *self {
            Query::Point(ref k) => k < &range.end,
            Query::Range(ref r) => r.end > range.start && r.start < range.end,
        }
    }
}

type ItemVec = SmallVec<[(usize, usize); 16]>;

pub struct QueryIter<'a, K: 'a, V: 'a> {
    tree: &'a IntervalTree<K, V>,
    items: ItemVec,
    query: Query<K>,
}

impl<'a, K: Ord + Clone, V> Clone for QueryIter<'a, K, V> {
    fn clone(&self) -> Self {
        QueryIter {
            tree: self.tree,
            items: self.items.clone(),
            query: self.query.clone(),
        }
    }
}

impl<'a, K: Ord + Clone + Debug, V: Debug> Debug for QueryIter<'a, K, V> {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        let v: Vec<_> = (*self).clone().collect();
        write!(fmt, "{:?}", v)
    }
}

impl<'a, K: Ord, V> Iterator for QueryIter<'a, K, V> {
    type Item = &'a Element<K, V>;

    fn next(&mut self) -> Option<&'a Element<K, V>> {
        while let Some((s, l)) = self.items.pop() {
            let i = s + l/2;

            let node = &self.tree.data[i];
            if self.query.point() < &node.max {
                {
                    let leftsz = i - s;
                    if leftsz > 0 {
                        self.items.push((s, leftsz));
                    }
                }

                if self.query.go_right(&node.element.range.start) {
                    {
                        let rightsz = l + s - i - 1;
                        if rightsz > 0 {
                            self.items.push((i + 1, rightsz));
                        }
                    }

                    if self.query.intersect(&node.element.range) {
                        return Some(&node.element);
                    }
                }
            }
        }
        None
    }
}