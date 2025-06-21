///// A munger which XORs a key with some data
//#[derive(Clone)]
//pub struct Xorcism<'a> {
//    // This field is just to suppress compiler complaints;
//    // feel free to delete it at any point.
//    _phantom: std::marker::PhantomData<&'a u8>,
//}
//
//impl<'a> Xorcism<'a> {
//    /// Create a new Xorcism munger from a key
//    ///
//    /// Should accept anything which has a cheap conversion to a byte slice.
//    pub fn new<Key>(key: &Key) -> Xorcism<'a> {
//        todo!()
//    }
//
//    /// XOR each byte of the input buffer with a byte from the key.
//    ///
//    /// Note that this is stateful: repeated calls are likely to produce different results,
//    /// even with identical inputs.
//    pub fn munge_in_place(&mut self, data: &mut [u8]) {
//        todo!()
//    }
//
//    /// XOR each byte of the data with a byte from the key.
//    ///
//    /// Note that this is stateful: repeated calls are likely to produce different results,
//    /// even with identical inputs.
//    ///
//    /// Should accept anything which has a cheap conversion to a byte iterator.
//    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
//    pub fn munge<Data>(&mut self, data: Data) -> impl Iterator<Item = u8> {
//        todo!();
//        // this empty iterator silences a compiler complaint that
//        // () doesn't implement ExactSizeIterator
//        std::iter::empty()
//    }
//}

use std::borrow::Borrow;
use std::cell::Cell;

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: &'a [u8],
    position: Cell<usize>,
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key: AsRef<[u8]> + ?Sized>(key: &'a Key) -> Xorcism<'a> {
        Xorcism {
            key: key.as_ref(),
            position: Cell::new(0),
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        let mut pos = self.position.get();
        for byte in data.iter_mut() {
            *byte ^= self.key[pos];
            pos = (pos + 1) % self.key.len();
        }
        self.position.set(pos);
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<Data>(&mut self, data: Data) -> impl Iterator<Item = u8>
    where
        Data: IntoIterator,
        Data::Item: std::borrow::Borrow<u8>,
    {
        MungeIterator {
            key: self.key,
            position: &self.position,
            data_iter: data.into_iter(),
        }
    }
}

pub struct MungeIterator<'a, I> {
    key: &'a [u8],
    position: &'a Cell<usize>,
    data_iter: I,
}

impl<'a, I> Iterator for MungeIterator<'a, I>
where
    I: Iterator,
    I::Item: std::borrow::Borrow<u8>,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.data_iter.next().map(|byte| {
            let pos = self.position.get();
            let result = *byte.borrow() ^ self.key[pos];
            self.position.set((pos + 1) % self.key.len());
            result
        })
    }
}
