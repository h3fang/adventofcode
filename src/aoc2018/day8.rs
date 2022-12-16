struct TreeNode {
    children: Vec<TreeNode>,
    metadata: Vec<i32>,
}

fn parse(data: &str) -> Vec<i32> {
    data.split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn parse_tree(mut nums: &[i32]) -> (&[i32], TreeNode) {
    assert!(nums.len() >= 2);
    let m = nums[0] as usize;
    let n = nums[1] as usize;
    nums = &nums[2..];
    let children = (0..m)
        .map(|_| {
            let (rest, node) = parse_tree(nums);
            nums = rest;
            node
        })
        .collect();

    assert!(nums.len() >= n);
    let metadata = nums[..n].to_vec();
    (&nums[n..], TreeNode { children, metadata })
}

fn part1(root: &TreeNode) -> i64 {
    root.children.iter().map(part1).sum::<i64>()
        + root.metadata.iter().map(|&m| m as i64).sum::<i64>()
}

fn part2(root: &TreeNode) -> i64 {
    if root.children.is_empty() {
        root.metadata.iter().map(|&m| m as i64).sum::<i64>()
    } else {
        root.metadata
            .iter()
            .map(|&m| {
                let i = m as usize - 1;
                if i < root.children.len() {
                    part2(&root.children[i])
                } else {
                    0
                }
            })
            .sum::<i64>()
    }
}

pub fn main() {
    let data = std::fs::read_to_string("data/2018/day8").unwrap();
    let nums = parse(&data);
    let (_, root) = parse_tree(&nums);
    println!("part1: {}", part1(&root));
    println!("part2: {}", part2(&root));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2".to_string();
        let nums = parse(&data);
        let (rest, root) = parse_tree(&nums);
        assert!(rest.is_empty());
        assert_eq!(138, part1(&root));
        assert_eq!(66, part2(&root));
    }
}
