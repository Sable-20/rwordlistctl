use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, RwLock,
};

use crate::units;
use crate::units::Units;

pub struct Data {
    size: AtomicUsize,
    // human_readable_size: AtomicUsize,
    unit: Arc<RwLock<Units>>,
    chunk_size: AtomicUsize,
}

// TODO: Implement custom error types

impl Data {
    pub fn new(size: usize, chunk_size: usize, unit: Units) -> Self {
        Self {
            size: AtomicUsize::new(size),
            // human_readable_size: AtomicUsize::new(units::readable_size(size).0.floor() as usize),
            unit: Arc::new(RwLock::new(unit)),
            chunk_size: AtomicUsize::new(chunk_size),
            // chunks: Arc::new(RwLock::new(
            //     Vec::<(AtomicUsize, AtomicUsize)>::with_capacity(chunk_count),
            // )),
        }
    }

    pub fn set_unit(&mut self, unit: Units) {
        *self.unit.write().unwrap() = unit;
    }

    pub fn get_unit(&self) -> Units {
        *self.unit.read().unwrap()
    }

    pub fn get_size(&self) -> usize {
        self.size.load(Ordering::Relaxed)
    }

    // pub fn get_human_readable_size(&self) -> usize {
    //     self.human_readable_size.load(Ordering::Relaxed)
    // }

    // pub fn set_human_readable_size(&mut self) {
    //     self.human_readable_size.store(
    //         crate::units::readable_size(self.get_size()).0 as usize,
    //         Ordering::Relaxed,
    //     );
    // }

    // pub fn chunk_data(&mut self) {
    //     let offset = self.size.load(Ordering::Relaxed) % self.chunk_count.load(Ordering::Relaxed);
    //     let chunk_size =
    //         self.size.load(Ordering::Relaxed) % self.chunk_count.load(Ordering::Relaxed);

    //     *self.chunks.write().unwrap() = (0..self.chunk_count.load(Ordering::Relaxed))
    //         .map(|counter| match counter {
    //             0 => (
    //                 AtomicUsize::new(counter * chunk_size),
    //                 AtomicUsize::new((counter + 1) * chunk_size),
    //             ),
    //             _ if counter == self.chunk_count.load(Ordering::Relaxed) - 1 => (
    //                 AtomicUsize::new(counter * chunk_size + 1),
    //                 AtomicUsize::new((counter + 1) * chunk_size + offset),
    //             ),
    //             _ => (
    //                 AtomicUsize::new(counter * chunk_size + 1),
    //                 AtomicUsize::new((counter + 1) * chunk_size),
    //             ),
    //         })
    //         .collect::<Vec<(AtomicUsize, AtomicUsize)>>();
    // }
}
