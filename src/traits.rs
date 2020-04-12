use std::collections;
use std::fmt;
use std::path::Path;

pub trait Edge<VI> {
    fn src(self) -> VI;
    fn dst(self) -> VI;
}

pub trait WeightedEdge<W> {
    fn weight(self) -> W;
}

pub trait IndexedEdge<EI> {
    fn index(self) -> EI;
}

pub trait LabeledEdge<K, V> {
    fn labels(self) -> collections::HashMap<K, V>;
}

pub trait Graph<'a> {
    type VI: num::PrimInt + 'a;
    type EI: num::PrimInt + 'a;
    type VIT: Iterator<Item = &'a Self::VI>;
    fn nv(&self) -> Self::VI;
    fn ne(&self) -> Self::EI;

    fn outneighbors(&'a self, v: Self::VI) -> Self::VIT;
    fn degree(&self, v: Self::VI) -> Self::VI;
    fn from_edge_file(f: &Path) -> Self;
}

pub trait DirectedGraph<'a, VI, VIT>
where
    VI: num::PrimInt,
    VIT: Iterator<Item = VI>,
{
    fn inneighbors(&'a self, v: VI) -> VIT;
}

pub trait NodeID: graph_matrix::MxElement + fmt::Display {}

impl NodeID for u8 {}
impl NodeID for u16 {}
impl NodeID for u32 {}
impl NodeID for u64 {}
