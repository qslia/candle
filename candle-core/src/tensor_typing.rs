#![allow](clippy::redundant_closure_call)
use crate::backend::{BackendDevice, BackendStorage};
use crate::op::{BackpropOp, BinaryOp, CmpOp, Op, ReduceOp, UnaryOp};
use crate::scalar::TensorOrScalar;
use crate::shape::{Dim, Dims, ShapeWithOneHole};
use crate::{bail, storage::Storage, DType, Device, Error, Layout, Result, Shape};
use std::sync::{Arc, RwLock};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]

pub struct TensorId(usize);

impl TensorId {
    fn new() -> Self {
        use std::sync::atomic;
        static COUNTER: atomic::AtomicUsize = atomic::AtomicUsize::new(1);
        Self(COUNTER.fetch_add(1, atomic::Ordering::Relaxed))
    }
}
