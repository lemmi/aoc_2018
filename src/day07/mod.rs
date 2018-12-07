use std::collections::{HashMap,BTreeSet,BinaryHeap};
use std::io;
use std::cmp::{Ordering,PartialOrd};
use std::iter::FromIterator;

use super::*;

type Deps = BTreeSet<char>;

#[derive(Clone,Debug,Eq,PartialEq)]
struct Node {
    name: char,
    dep: Deps,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        let p = if self.dep == other.dep {
            Some(self.name.cmp(&other.name))
        } else  if self.dep.contains(&other.name) {
            Some(Ordering::Greater)
        } else if other.dep.contains(&self.name) {
            Some(Ordering::Less)
        } else {
            None
        };
        p
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> ", self.name)?;
        let mut sep = "";
        for dep in self.dep.iter() {
            write!(f, "{}{}", sep, dep)?;
            sep=", ";
        }
        Ok(())
    }
}

impl Node {
    fn new(name: char) -> Node {
        Node{name: name, dep: BTreeSet::new()}
    }
//    fn with_deps(name: char, deps: &Vec<char>) -> Node {
//        Node{name: name, dep: deps.clone()}
//    }
    fn add(&mut self, dep: char) {
        self.dep.insert(dep);
    }
    fn satisfied(&self, deps: impl Iterator<Item = char>) -> bool {
        self.dep.is_subset(&BTreeSet::from_iter(deps))
    }
}

struct Graph {
    edges: HashMap<char,Node>,
}

impl Graph {
    fn new() -> Graph {
        Graph{edges: HashMap::new()}
    }

    fn build(lines: impl Iterator<Item = io::Result<String>>) -> Result<Graph,StarError> {
        let mut g = Self::new();
        for r in lines.map(|l| l.map_err(|e| StarError::from(e))) {
            let s = r?;
            let w: Vec<_> = s.split_whitespace().collect();
            if w.len() != 10 {
                return Err(format!("Wrong number of words: \"{:?}\"", w).into());
            }
            let dep = w[1].chars().next().expect("Expected single char dep name");
            let node = w[7].chars().next().expect("Expected single char dep name");
            g.add(dep, node);
        }
        Ok(g)
    }

    fn add(&mut self, dep: char, node: char) {
        self.edges.entry(dep).or_insert(Node::new(dep));
        let v = self.edges.entry(node).or_insert(Node::new(node));
        v.add(dep);
    }
}
impl Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for v in self.edges.values() {
            writeln!(f, "{}", v)?;
        }
        Ok(())
    }
}

fn next_work(todo: &Vec<Node>, done: &Vec<Node>) -> Option<usize> {
    todo.iter().enumerate()
        .filter(|(_,m)| m.satisfied(done.iter().map(|n| n.name)))
        .min_by(|(_,a), (_,b)| a.partial_cmp(b).unwrap_or(a.name.cmp(&b.name)))
        .map(|(i,_)| i)
}

fn toposort(v: &Vec<Node>) -> Vec<Node> {
    let mut done: Vec<Node> = Vec::new();
    let mut todo = v.to_owned();
    while todo.len() > 0 {
        let m = next_work(&todo, &done);

        match m {
            Some(i) => {
                let n = todo.swap_remove(i);
                done.push(n);
            },
            None => panic!("Loop?"),
        }
    }
    done
}

pub fn star1(lines: impl Iterator<Item = io::Result<String>>) -> StarResult {
    let g = Graph::build(lines)?;
    let v: Vec<Node> = g.edges.values().cloned().collect();
    let v = toposort(&v);
    println!("{}",v.iter().map(|n| n.name).collect::<String>());
    Ok(())
}

#[derive(PartialEq,Eq,Debug)]
struct Work {
    node: Node,
    t: usize
}

impl Work {
    fn new(n: Node, starttime: usize) -> Work{
        Work { node: n, t: starttime }
    }
    fn finish_in(&self) -> usize {
        self.t + self.worksize()
    }
    fn worksize(&self) -> usize {
        self.node.name as usize - 'A' as usize + 61
    }
}

impl Ord for Work {
    fn cmp(&self, other: &Work) -> Ordering {
        other.finish_in().cmp(&self.finish_in())
    }
}
impl PartialOrd for Work {
    fn partial_cmp(&self, other: &Work) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parallel_topo(v: &Vec<Node>, worker: usize) -> (Vec<Node>, usize) {
    let mut q = BinaryHeap::new();
    let mut todo = v.to_owned();
    let mut t = 0;
    let mut done = Vec::new();

    while todo.len() > 0 || q.len() > 0{
        while q.len() < worker {
            match next_work(&todo, &done) {
                Some(i) => { 
                    let n = todo.swap_remove(i);
                    q.push(Work::new(n, t));
                },
                None => break,
            }
        }
        if let Some(d) = q.pop() {
            t = d.finish_in();
            done.push(d.node);
        } else {
            panic!("Loop?");
        }
    }

    (done, t)
}

pub fn star2(lines: impl Iterator<Item = std::io::Result<String>>) -> super::StarResult {
    let g = Graph::build(lines)?;
    let v: Vec<Node> = g.edges.values().cloned().collect();
    let (v, t) = parallel_topo(&v, 5);
    println!("Final order: {} finished in {}",v.iter().map(|n| n.name).collect::<String>(), t);
    Ok(())
}
