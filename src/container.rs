use std::fmt::{ Debug, Formatter, Result };

use num::traits::{ One };

use util::{ self, ExtInt, Halveable };
use store::Store::{ self, Array, Bitmap };

#[derive(PartialEq, Clone)]
pub struct Container<Size: ExtInt> {
    key: Size,
    len: u64,
    store: Store<Size>,
}

impl<Size: ExtInt> Container<Size> {
    pub fn new(key: Size) -> Container<Size> {
        Container {
            key: key,
            len: 0,
            store: Array(Vec::new()),
        }
    }
}
impl<Size: ExtInt + Halveable> Container<Size> {
    pub fn from_raw64(bits: &Vec<u64>, next: usize) -> Container<<Size as util::Halveable>::HalfSize> {
        let key = util::cast(bits[next]);
        let len = bits[next+1];
        let store_type = bits[next+2];
        let size = bits[next+3];
        if store_type == 0 {
            let mut vs = Vec::new();
            for i in next+4..next+4+(size as usize) {
                vs.push(util::cast(bits[i]));
            }
            return Container{
                key: key,
                len: size,
                store: Array(vs)
            };
        } else {
            let mut vs = Vec::new();
            for i in next+4..next+4+(size as usize) {
                vs.push(bits[i]);
            }
            return Container{
                key: key,
                len: len,
                store: Bitmap(vs.into_boxed_slice())
            };
        }
    }
}

impl<Size: ExtInt> Container<Size> {
    #[inline]
    pub fn key(&self) -> Size { self.key }

    #[inline]
    pub fn len(&self) -> u64 { self.len }

    #[inline]
    pub fn insert(&mut self, index: Size) -> bool {
        if self.store.insert(index) {
            self.len = self.len + 1;
            self.ensure_correct_store();
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn remove(&mut self, index: Size) -> bool {
        if self.store.remove(index) {
            self.len = self.len - 1;
            self.ensure_correct_store();
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn contains(&self, index: Size) -> bool {
        self.store.contains(index)
    }

    #[inline]
    pub fn iter<'a>(&'a self) -> Box<Iterator<Item = Size> + 'a> {
        self.store.iter()
    }

    #[inline]
    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.store.is_disjoint(&other.store)
    }

    #[inline]
    pub fn is_subset(&self, other: &Self) -> bool {
        if self.len > other.len {
            false
        } else {
            self.store.is_subset(&other.store)
        }
    }

    #[inline]
    pub fn union_with(&mut self, other: &Self) {
        self.store.union_with(&other.store);
        self.len = self.store.len();
        self.ensure_correct_store();
    }

    #[inline]
    pub fn intersect_with(&mut self, other: &Self) {
        self.store.intersect_with(&other.store);
        self.len = self.store.len();
        self.ensure_correct_store();
    }

    #[inline]
    pub fn difference_with(&mut self, other: &Self) {
        self.store.difference_with(&other.store);
        self.len = self.store.len();
        self.ensure_correct_store();
    }

    #[inline]
    pub fn symmetric_difference_with(&mut self, other: &Self) {
        self.store.symmetric_difference_with(&other.store);
        self.len = self.store.len();
        self.ensure_correct_store();
    }

    #[inline]
    pub fn min(&self) -> Size {
        self.store.min()
    }

    #[inline]
    pub fn max(&self) -> Size {
        self.store.max()
    }

    #[inline]
    fn ensure_correct_store(&mut self) {
        let limit = util::cast(<Size as One>::one().rotate_right(4));
        let new_store = match (&self.store, self.len) {
            (store @ &Bitmap(..), len) if len <= limit => Some(store.to_array()),
            (store @ &Array(..), len) if len > limit => Some(store.to_bitmap()),
            _ => None,
        };
        if let Some(new_store) = new_store {
            self.store = new_store;
        }
    }

    #[inline]
    pub fn to_raw64(&self) -> Vec<u64> {
        // key, num_entries, type, size_of_vals_block, vals
        let mut raw64: Vec<u64> = Vec::new();
        raw64.push(util::cast(self.key));
        raw64.push(self.len);
        match self.store {
            Array(ref vec) => {
                raw64.push(0);
                raw64.push(vec.len()  as u64);
                for e in vec {
                    raw64.push(e.to64());
                }
            },
            Bitmap(ref bits) => {
                raw64.push(1);
                raw64.push(bits.len() as u64);
                raw64.extend_from_slice(bits);
            }
        }
        return raw64;
    }

}

impl<Size: ExtInt + Debug> Debug for Container<Size> {
    #[inline]
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        format!("Container<{:?} @ {:?}>", self.len(), self.key()).fmt(formatter)
    }
}
