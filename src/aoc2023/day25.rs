use ahash::HashMap;
use rand::prelude::*;
use rayon::prelude::*;

fn parse(data: &str) -> HashMap<usize, (Vec<usize>, usize)> {
    let mut map: HashMap<&str, usize> = HashMap::default();
    let mut graph: HashMap<usize, (Vec<usize>, usize)> = HashMap::default();
    data.trim().lines().for_each(|l| {
        let (a, vs) = l.split_once(": ").unwrap();
        let n = map.len();
        let u = *map.entry(a).or_insert(n);
        let vs = vs
            .split_ascii_whitespace()
            .map(|b| {
                let n = map.len();
                let v = *map.entry(b).or_insert(n);
                graph.entry(v).or_insert((vec![], 1)).0.push(u);
                v
            })
            .collect::<Vec<_>>();
        graph.entry(u).or_insert((vec![], 1)).0.extend(vs);
    });
    graph
}

fn contract(mut graph: HashMap<usize, (Vec<usize>, usize)>) -> Option<usize> {
    let mut rng = thread_rng();

    // Karger's algorithm
    // safety: different keys (u and v) of the hash map are accessed separately
    while graph.len() > 2 {
        let u = *graph.keys().choose(&mut rng).unwrap();
        let (vs1, m) = {
            let (vs, m) = graph.get_mut(&u).unwrap();
            (vs as *mut Vec<usize>, m as *mut usize)
        };
        let v = unsafe { *(*vs1).choose(&mut rng).unwrap() };
        let (vs2, n) = graph.get(&v).unwrap();
        unsafe {
            (*vs1).extend(vs2);
            (*vs1).retain(|&e| e != u && e != v);
            *m += n;
        }

        for n in unsafe { &*vs1 } {
            let vs = &mut graph.get_mut(n).unwrap().0;
            vs.iter_mut().for_each(|e| {
                if *e == v {
                    *e = u;
                }
            });
        }

        graph.remove(&v);
    }
    if graph.values().next().unwrap().0.len() == 3 {
        Some(graph.values().map(|v| v.1).product())
    } else {
        None
    }
}

fn part1(graph: HashMap<usize, (Vec<usize>, usize)>) -> usize {
    (0..)
        .par_bridge()
        .find_map_any(|_| contract(graph.clone()))
        .unwrap()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2023/day25").unwrap();
    let graph = parse(&data);
    println!("part1: {}", part1(graph));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = r"
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
        let graph = parse(data);
        assert_eq!(54, part1(graph));
    }
}
