use std::{collections::HashMap, ops::Deref};

use crate::get_input;

#[derive(PartialEq)]
enum NodeType {
	File,
	Directory,
}

struct Metadata {
	size: usize,
	ntype: NodeType,
}

impl Metadata {
	fn new(ntype: NodeType, size: usize) -> Self {
		Self { size, ntype }
	}

	fn is_file(&self) -> bool {
		self.ntype == NodeType::File
	}

	fn is_dir(&self) -> bool {
		self.ntype == NodeType::Directory
	}
}

struct Node {
	parent: Option<*mut Self>,
	meta: Metadata,
	children: Option<HashMap<String, Self>>,
}

impl Node {
	fn new(ntype: NodeType, size: usize, parent: Option<*mut Self>) -> Self {
		let children = match ntype {
			NodeType::File => None,
			NodeType::Directory => Some(HashMap::new()),
		};
		Self {
			parent,
			meta: Metadata::new(ntype, size),
			children,
		}
	}

	fn is_orphan(&self) -> bool {
		self.parent.is_none()
	}

	fn children(&self) -> &HashMap<String, Self> {
		self.children.as_ref().unwrap()
	}

	fn children_mut(&mut self) -> &mut HashMap<String, Self> {
		self.children.as_mut().unwrap()
	}

	fn size(&self) -> usize {
		self.meta.size
		// For being iterative:
		// match self.meta.ntype {
		// 	NodeType::File => self.meta.size,
		// 	NodeType::Directory => self.children
		// 		.into_iter()
		// 		.map(|node| {
		// 			node
		// 			.values()
		// 			.map(|child| child.size())
		// 			.sum::<usize>()
		// 		})
		// 		.sum(),
		// }
	}
}

struct FileSystem {
	root: Node,
	cwd: *mut Node,
}

impl FileSystem {
	fn new() -> Self {
		let mut root = Node::new(NodeType::Directory, 0, None);
		// SAFETY: We'll always have a root for as long as the filesystem is alive.
		let cwd: *mut Node = &mut root as *mut Node;
		Self { root, cwd }
	}

	fn get_cwd(&mut self) -> &mut Node {
		// SAFETY: We'll always have a root for as long as the filesystem is alive.
		unsafe { self.cwd.as_mut().unwrap() }
	}

	fn create_file(&mut self, path: &str, size: usize) -> bool {
		let cwd = self.get_cwd();

		let node = Node::new(NodeType::File, size, Some(cwd as *mut Node));
		cwd.meta.size += size;

		cwd.children_mut().insert(path.to_string(), node).is_some()
	}

	fn create_dir(&mut self, path: &str) -> bool {
		let cwd = self.get_cwd();

		let node = Node::new(NodeType::Directory, 0, Some(cwd as *mut Node));

		// SAFETY: We guarantee `cwd` is a directory in `self.cd`
		cwd.children_mut().insert(path.to_string(), node).is_some()
	}

	fn cd(&mut self, path: &str) -> bool {
		let parent = unsafe { self.cwd.as_ref().unwrap().parent.as_ref() };
		if parent.is_some() && path == ".." {
			self.cwd = *parent.unwrap();
			return true;
		}

		let cwd = self.get_cwd();

		match cwd.children_mut().get_mut(path) {
			Some(nwd) => match nwd.meta.ntype {
				NodeType::File => false,
				NodeType::Directory => {
					self.cwd = nwd as *mut Node;
					true
				}
			},
			None => false,
		}
	}
}

#[test]
fn test_part1() {
	let input = get_input("day_7_test.txt");
	let mut fs = FileSystem::new();
}

#[test]
fn part1() {}

#[test]
fn part2() {}
