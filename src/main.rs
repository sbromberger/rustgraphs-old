use rustgraph::traits::Graph;
use rustgraph::SimpleGraph;
use std::env;
use std::path::Path;
use std::time::Instant;

const NRUNS: usize = 50;
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let src = &args[2];
    let src: u32 = src.parse().expect("invalid source");

    let now = Instant::now();
    let h: SimpleGraph<u32> = SimpleGraph::from_edge_file(Path::new(filename));
    println!("Load took {}ms", now.elapsed().as_micros() as f64 / 1000.0);
    println!("h = {}", h);
    let mut avg: f64 = 0.0;

    for _ in 0..NRUNS {
        let now = Instant::now();
        let levels = h.bfs(src);
        let elp = now.elapsed().as_micros() as f64 / 1000.0;
        avg += elp;
        // println!("BFS took {}ms", elp);
        // println!(
        //     "max level = {}",
        //     levels
        //         .into_iter()
        //         .filter(|&x| { x < std::u32::MAX })
        //         .max()
        //         .unwrap()
        // );
    }
    println!(
        "bfs unstable sort: average over {} runs: {:.3}ms",
        NRUNS,
        avg / NRUNS as f64
    );

    // avg = 0.0;
    // for _ in 0..NRUNS {
    //     let now = Instant::now();
    //     let levels = h.bfs_iter(src);
    //     let elp = now.elapsed().as_micros() as f64 / 1000.0;
    //     avg += elp;
    //     // println!("BFS iter took {}ms", elp);
    //     // println!(
    //     //     "max level = {}",
    //     //     levels
    //     //         .into_iter()
    //     //         .filter(|&x| { x < U32_MAX })
    //     //         .max()
    //     //         .unwrap()
    //     // );
    // }
    // println!(
    //     "bfsiter: average over {} runs: {:.3}ms",
    //     NRUNS,
    //     avg / NRUNS as f64
    // );

    // avg = 0.0;
    // for _ in 0..NRUNS {
    //     let now = Instant::now();
    //     let levels = h.bfs_unsorted(src);
    //     let elp = now.elapsed().as_micros() as f64 / 1000.0;
    //     avg += elp;
    //     // println!("BFS rdx took {}ms", elp);
    //     // println!(
    //     //     "max level = {}",
    //     //     levels
    //     //         .into_iter()
    //     //         .filter(|&x| { x < U32_MAX })
    //     //         .max()
    //     //         .unwrap()
    //     // );
    // }
    // println!(
    //     "unsorted: average over {} runs: {:.3}ms",
    //     NRUNS,
    //     avg / NRUNS as f64
    // );
}
