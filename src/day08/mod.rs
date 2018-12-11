use super::*;

#[derive(Debug)]
struct Node<'a> {
    children: Vec<Node<'a>>,
    metadata: &'a [u32],
}

impl<'a> Node<'a> {
    fn build(s: &[u32]) -> Result<(Node, usize), StarError> {
        if s.len() < 2 {
            return Err(format!("Node to small: {:?}", s).into());
        }

        let n_children = s[0] as usize;
        let n_metadata = s[1] as usize;

        let mut data = &s[2..s.len() - n_metadata];

        if n_children == 0 {
            let metadata = &s[2..2 + n_metadata];
            Ok((
                Node {
                    children: Vec::new(),
                    metadata,
                },
                2 + n_metadata,
            ))
        } else {
            let mut children = Vec::new();
            for _ in 0..n_children {
                let (child, c_size) = Node::build(data)?;
                children.push(child);
                data = &data[c_size..];
            }

            let size = s.len() - data.len();
            Ok((
                Node {
                    children,
                    metadata: &s[size - n_metadata..size],
                },
                s.len() - data.len(),
            ))
        }
    }

    fn sum_metadata(&self) -> u32 {
        self.children.iter().map(|c| c.sum_metadata()).sum::<u32>()
            + self.metadata.iter().sum::<u32>()
    }

    fn get_value(&self) -> u32 {
        if self.children.is_empty() {
            self.sum_metadata()
        } else {
            let mut value = 0;
            for &idx in self.metadata.iter() {
                if idx == 0 {
                    continue;
                }

                if let Some(child) = self.children.get(idx as usize - 1) {
                    value += child.get_value();
                }
            }
            value
        }
    }
}

fn parse_input(
    mut lines: impl Iterator<Item = std::io::Result<String>>,
) -> Result<Vec<u32>, StarError> {
    lines
        .next()
        .ok_or("Expected one line input")??
        .split_whitespace()
        .map(|s| s.parse::<u32>().map_err(StarError::from))
        .collect::<Result<Vec<_>, _>>()
}

pub fn star1(lines: impl Iterator<Item = std::io::Result<String>>) -> StarResult {
    let input = parse_input(lines)?;
    let (tree, _) = Node::build(&input)?;
    println!("Sum of metadata: {}", tree.sum_metadata());
    Ok(())
}

pub fn star2(lines: impl Iterator<Item = std::io::Result<String>>) -> StarResult {
    let input = parse_input(lines)?;
    let (tree, _) = Node::build(&input)?;
    println!("Value of root: {}", tree.get_value());
    Ok(())
}
