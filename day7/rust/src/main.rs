use std::{cmp::Ordering, iter::Peekable, str::FromStr};

const INPUT_FILE: &str = "../input.txt";

fn main() -> Result<(), std::io::Error> {
    part_1()?;
    // part_2()?;
    Ok(())
}

#[derive(Debug)]
struct File<'a> {
    name: &'a str,
    size: usize,
}

#[derive(Debug)]
struct Dir<'a> {
    name: &'a str,
    content: Vec<VFSNode<'a>>,
}

#[derive(Debug)]
enum VFSNode<'a> {
    File(File<'a>),
    Dir(Dir<'a>),
}

#[derive(Default)]
struct VFS<'a> {
    root: Vec<VFSNode<'a>>,
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
                b"cd" => match &bytes[5..7] {
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
                                        name,
                                        content: Vec::new(),
                                    };
                                    VFSNode::Dir(dir)
                                }
                                size => {
                                    let file = File {
                                        name,
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

impl<'a> FromStr for VFS<'a> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vfs = VFS::default();
        let mut lines = s.lines().peekable();
        // skip fires command
        let _ = lines.next();
        while let Some(command) = Command::extract(&mut lines) {
            println!("{command:?}");
        }
        Ok(vfs)
    }
}

fn part_1() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string(INPUT_FILE)?;
    let vfs: VFS = input.parse().unwrap();
    Ok(())
}
