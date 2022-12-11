use std::{collections::HashMap, iter::Peekable};

const INPUT_FILE: &str = "../input.txt";

fn main() -> Result<(), std::io::Error> {
    part_1()?;
    part_2()?;
    Ok(())
}

enum Size {
    DirSize(Vec<usize>),
    FileSize(usize),
}

#[derive(Debug)]
struct File {
    size: usize,
}

#[derive(Debug)]
struct Dir<'a> {
    parent: usize,
    name: &'a str,
    content: Vec<usize>,
}

impl Dir<'_> {
    fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    fn collect_sizes(&self, vfs: &VFS) -> Size {
        let sizes = self
            .content
            .iter()
            .map(|node_id| vfs.nodes[*node_id].collect_sizes(vfs))
            .collect::<Vec<_>>();
        let self_size = sizes
            .iter()
            .map(|size| match size {
                Size::FileSize(size) => size,
                Size::DirSize(dir_sizes) => dir_sizes.last().unwrap(),
            })
            .sum::<usize>();
        let mut sizes = sizes
            .into_iter()
            .filter_map(|size| match size {
                Size::FileSize(_) => None,
                Size::DirSize(sizes) => Some(sizes),
            })
            .flatten()
            .collect::<Vec<_>>();
        sizes.push(self_size);
        Size::DirSize(sizes)
    }
}

#[derive(Debug)]
enum VFSNode<'a> {
    File(File),
    Dir(Dir<'a>),
}

impl VFSNode<'_> {
    fn collect_sizes(&self, vfs: &VFS) -> Size {
        match self {
            VFSNode::File(file) => Size::FileSize(file.size),
            VFSNode::Dir(dir) => dir.collect_sizes(vfs),
        }
    }
}

#[derive(Debug, Default)]
struct VFS<'a> {
    cwd: usize,
    nodes: Vec<VFSNode<'a>>,
    dir_names_to_ids: HashMap<(usize, &'a str), usize>,
}

impl<'a> VFS<'a> {
    fn new() -> Self {
        Self {
            cwd: 0,
            nodes: vec![VFSNode::Dir(Dir {
                parent: 0,
                name: "/",
                content: Vec::new(),
            })],
            dir_names_to_ids: HashMap::from_iter([((0, "/"), 0)].into_iter()),
        }
    }

    fn from_str(s: &'a str) -> Self {
        let mut vfs = VFS::new();
        let mut lines = s.lines().peekable();
        while let Some(command) = Command::extract(&mut lines) {
            vfs.execute(command);
        }
        vfs
    }

    fn cwd_empty(&self) -> bool {
        match self.nodes[self.cwd] {
            VFSNode::Dir(ref dir) => dir.is_empty(),
            _ => unreachable!(),
        }
    }

    fn cwd<'b>(&'b mut self) -> &'b Dir<'a>
    where
        'a: 'b,
    {
        match self.nodes[self.cwd] {
            VFSNode::Dir(ref dir) => dir,
            _ => unreachable!(),
        }
    }

    fn cwd_mut<'b>(&'b mut self) -> &'b mut Dir<'a>
    where
        'a: 'b,
    {
        match self.nodes[self.cwd] {
            VFSNode::Dir(ref mut dir) => dir,
            _ => unreachable!(),
        }
    }

    fn execute(&mut self, command: Command<'a>) {
        match command {
            Command::Ls(ls) => {
                if self.cwd_empty() {
                    for mut node in ls {
                        let new_id = self.nodes.len();

                        match node {
                            VFSNode::Dir(ref mut ls_dir) => {
                                ls_dir.parent = self.cwd;
                                self.dir_names_to_ids
                                    .insert((self.cwd, ls_dir.name), new_id);
                            }
                            VFSNode::File(_) => {}
                        }

                        self.nodes.push(node);
                        self.cwd_mut().content.push(new_id);
                    }
                }
            }
            Command::Cd(cd) => match cd {
                    Cd::Up => self.cwd = self.cwd().parent,
                    Cd::Dir(dir_name) => {
                        self.cwd = self.dir_names_to_ids[&(self.cwd, dir_name)];
                    }
                },
        }
    }

    fn all_dir_sizes(&self) -> Vec<usize> {
        if let Size::DirSize(sizes) = self.nodes[0].collect_sizes(self) {
            sizes
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug)]
enum Command<'a> {
    Ls(Vec<VFSNode<'a>>),
    Cd(Cd<'a>),
}

#[derive(Debug)]
enum Cd<'a> {
    Up,
    Dir(&'a str),
}

impl<'a> Command<'a> {
    fn extract(lines: &mut Peekable<impl Iterator<Item = &'a str>>) -> Option<Self> {
        lines.next().map(|line| {
            let bytes = line.as_bytes();
            match &bytes[2..4] {
                b"cd" => match &bytes[5..] {
                    b".." => Self::Cd(Cd::Up),
                    _ => Self::Cd(Cd::Dir(std::str::from_utf8(&bytes[5..]).unwrap())),
                },
                b"ls" => {
                    let mut nodes = Vec::new();
                    while let Some(line) = lines.peek() {
                        if line.starts_with('$') {
                            break;
                        } else {
                            let line = lines.next().unwrap();
                            let (id, name) = line.split_once(' ').unwrap();
                            let node = match id {
                                "dir" => {
                                    let dir = Dir {
                                        parent: 0,
                                        name,
                                        content: Vec::new(),
                                    };
                                    VFSNode::Dir(dir)
                                }
                                size => {
                                    let file = File {
                                        size: size.parse().unwrap(),
                                    };
                                    VFSNode::File(file)
                                }
                            };
                            nodes.push(node);
                        }
                    }
                    Self::Ls(nodes)
                }
                _ => unreachable!(),
            }
        })
    }
}

fn part_1() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string(INPUT_FILE)?;
    let vfs = VFS::from_str(&input);
    let sum = vfs
        .all_dir_sizes()
        .iter()
        .filter(|s| 100000 >= **s)
        .sum::<usize>();
    println!("sum: {sum}");
    Ok(())
}

fn part_2() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string(INPUT_FILE)?;
    let vfs = VFS::from_str(&input);
    let all_sizes = vfs.all_dir_sizes();
    let total = all_sizes.last().unwrap();
    let total_free = 70000000 - total;
    let need_to_free = 30000000 - total_free;

    let min_to_delete = all_sizes
        .iter()
        .filter(|s| **s > need_to_free)
        .min()
        .unwrap();
    println!("min_to_delete: {min_to_delete}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vfs() {
        let input = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";
        let vfs = VFS::from_str(input);
        let all_sizes = vfs.all_dir_sizes();
        let sizes = all_sizes
            .iter()
            .filter(|s| 100000 >= **s)
            .collect::<Vec<_>>();
        let sum = sizes.iter().copied().sum::<usize>();
        assert_eq!(sum, 95437);
    }
}
