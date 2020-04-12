use crate::traits::Graph;
use bitvec::prelude as bv;
use graph_matrix;
use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::mem;
use std::path::Path;

pub mod traits;

#[derive(Debug, Clone)]
pub struct SimpleGraph<T>
where
    T: traits::NodeID,
{
    fadj: graph_matrix::GraphMatrix<T>,
    badj: graph_matrix::GraphMatrix<T>,
}

impl<'a, T> traits::Graph<'a> for SimpleGraph<T>
where
    T: traits::NodeID + 'a,
{
    type VI = T;
    type EI = usize;
    type VIT = std::slice::Iter<'a, T>;

    fn from_edge_file(fname: &Path) -> Self {
        let f = File::open(fname).expect("Cannot open file");
        let file = BufReader::new(&f);
        let mut edgelist: Vec<(T, T)> = vec![];
        for line in file.lines() {
            let l = line.expect("error reading file"); // produces a std::string::String
            let l = l.trim(); // changes to &str
            if l.starts_with("#") {
                continue;
            }
            let mut eit = l.split_whitespace();
            let s1 = eit.next().expect("Invalid line (first field)");
            let s2 = eit.next().expect("Invalid line (second field)");
            if eit.next().is_some() {
                panic!("Invalid line (extra fields)");
            }
            let src128: u128 = s1.parse().unwrap();
            let dst128: u128 = s2.parse().unwrap();
            let src = T::from(src128).expect("vertex out of range");
            let dst = T::from(dst128).expect("vertex out of range");
            edgelist.push((src, dst));
        }
        let bedges = edgelist.clone().iter().map(|x| (x.1, x.0)).collect();
        let fadj = graph_matrix::GraphMatrix::from_edges(edgelist);
        let badj = graph_matrix::GraphMatrix::from_edges(bedges);
        SimpleGraph { fadj, badj }
    }

    fn nv(&self) -> T {
        T::from(self.fadj.dim()).unwrap()
    }

    fn ne(&self) -> Self::EI {
        self.fadj.n()
    }

    fn outneighbors(&'a self, v: T) -> Self::VIT
    where
        T: traits::NodeID,
    {
        let uv = v.to_usize().unwrap();
        self.fadj.row(uv)
    }

    fn degree(&self, v: T) -> T
    where
        T: traits::NodeID,
    {
        let uv = v.to_usize().expect("Invalid vertex");
        self.fadj.row_len(uv)
    }
}

impl<T> SimpleGraph<T>
where
    T: traits::NodeID,
{
    pub fn bfs(&self, src: T) -> Vec<T>
    where
        T: traits::NodeID,
    {
        let n = self.nv().to_usize().unwrap();
        let maxdeg = (1..n)
            .map(|v| self.degree(T::from(v).unwrap()).to_usize().unwrap())
            .max()
            .expect("Invalid degree found");
        let mut visited: bv::BitVec<bv::Lsb0, u64> = bv::BitVec::repeat(false, n);

        let mut levels: Vec<T> = vec![T::max_value(); n];
        let mut cur_level: Vec<T> = Vec::new();
        cur_level.reserve(maxdeg);

        let mut next_level: Vec<T> = Vec::new();
        next_level.reserve(maxdeg);

        let s = src.to_usize().expect("Invalid vertex");
        visited.set(s, true);
        cur_level.push(src);
        levels[s] = T::zero();

        let mut n_level = T::one();

        // println!("cur_level = {:?}", cur_level);
        while !cur_level.is_empty() {
            for v in cur_level.iter().cloned() {
                for i in self.outneighbors(v) {
                    // println!("neighbor {:?}", i);
                    let ui = i.to_usize().expect("Invalid vertex");
                    if unsafe { !*visited.get_unchecked(ui) } {
                        // println!("{:?} -> {}", v, ui);
                        next_level.push(*i);
                        unsafe {
                            visited.set_unchecked(ui, true);
                            *levels.get_unchecked_mut(ui) = n_level;
                        }
                    }
                }
            }
            n_level = n_level + T::one();
            // println!("next_level = {:?}", next_level);
            cur_level.clear();

            mem::swap(&mut cur_level, &mut next_level);
            cur_level.sort_unstable();
        }
        levels
    }
}

impl<T> fmt::Display for SimpleGraph<T>
where
    T: traits::NodeID,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}) Graph", self.nv(), self.ne())
    }
}
