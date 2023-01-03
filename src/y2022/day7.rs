use std::{collections::HashMap, ptr::null_mut};

use crate::get_input;

#[derive(Debug)]
enum Command<'a> {
	List,
	Change(&'a str),
}

impl<'a> TryFrom<&'a str> for Command<'a> {
	type Error = ();

	fn try_from(s: &'a str) -> Result<Self, Self::Error> {
		if !s.starts_with("$") {
			return Err(());
		}

		let mut s = s.split_whitespace().skip(1);

		match s.next().ok_or(())? {
			"ls" => Ok(Self::List),
			"cd" => Ok(Self::Change(s.next().ok_or(())?)),
			_ => Err(()),
		}
	}
}

#[derive(Debug)]
enum Input<'a> {
	Command(Command<'a>),
	Directory(&'a str),
	File(usize, &'a str),
}

impl<'a> TryFrom<&'a str> for Input<'a> {
	type Error = ();

	fn try_from(s: &'a str) -> Result<Self, Self::Error> {
		let mut words = s.split_whitespace();
		let start = words.next().ok_or(())?;

		match start {
			"$" => Ok(Self::Command(s.try_into()?)),
			"dir" => Ok(Self::Directory(words.next().ok_or(())?)),
			// Handle the case where the first word is a file size
			_ => {
				let size = start.parse::<usize>().map_err(|_| ())?;
				let name = words.next().ok_or(())?;
				Ok(Self::File(size, name))
			}
		}
	}
}

#[derive(Clone, Debug)]
enum PathError {
	AtRoot,
	NameContainsSlash,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Path {
	pstr: String,
}

impl Path {
	fn new() -> Self {
		Self { pstr: String::from("/") }
	}

	fn abs_path(&self) -> &str {
		&self.pstr
	}

	fn name(&self) -> &str {
		let mut iter = self.pstr.split('/');
		iter.next_back();
		// SAFETY: By this point, the files have names, except for root.
		iter.next_back().unwrap()
	}

	fn parent(&mut self) -> Result<&str, PathError> {
		if self.abs_path() == "/" {
			return Err(PathError::AtRoot);
		}

		self.pstr.pop(); // pop off ending "/"
		loop {
			if self.pstr.ends_with("/") {
				break;
			} else {
				self.pstr.pop();
			}
		}

		Ok(&self.pstr)
	}

	fn child(&mut self, name: &str) -> Result<&str, PathError> {
		if name.contains("/") {
			return Err(PathError::NameContainsSlash);
		}

		self.pstr.push_str(name);
		self.pstr.push('/');

		Ok(&self.pstr)
	}
}

type NodeID = usize;

#[derive(Clone, Debug)]
struct File {
	_inode: NodeID,
	_size: usize,
	path: Path,
}

impl File {
	fn new(inode: NodeID, size: usize, path: Path) -> Self {
		Self { _inode: inode, _size: size, path }
	}
}

#[derive(Clone, Debug)]
struct Directory {
	inode: NodeID,
	parent: Option<NodeID>,
	size: usize,
	path: Path,
	children: Vec<NodeID>,
}

impl Directory {
	fn new(inode: NodeID, size: usize, path: Path) -> Self {
		let children = Vec::new();
		Self {
			inode,
			size,
			path,
			children,
			parent: None,
		}
	}

	fn with_parent(mut self, parent: NodeID) -> Self {
		self.parent = Some(parent);
		self
	}

	fn parent_inode(&self) -> Option<NodeID> {
		self.parent
	}

	fn add_as_child(&mut self, inode: NodeID) {
		self.children.push(inode);
		self.children.sort_unstable();
	}

	fn children(&self) -> &Vec<NodeID> {
		&self.children
	}
}

#[derive(Clone, Debug)]
enum Node {
	File(File),
	Directory(Directory),
}

impl Node {
	fn abs_path(&self) -> &str {
		match self {
			Node::File(file) => file.path.abs_path(),
			Node::Directory(dir) => dir.path.abs_path(),
		}
	}

	fn name(&self) -> &str {
		match self {
			Node::File(file) => file.path.name(),
			Node::Directory(dir) => dir.path.name(),
		}
	}

	fn as_dir_mut(&mut self) -> Option<&mut Directory> {
		match self {
			Node::File(_) => None,
			Node::Directory(dir) => Some(dir),
		}
	}
}

#[derive(Clone, Debug)]
struct FileSystem {
	next_index: NodeID,
	cwd: *mut Directory,
	names: HashMap<String, NodeID>,
	nodes: HashMap<NodeID, Node>,
}

impl FileSystem {
	fn new() -> Self {
		let mut fs = Self {
			next_index: 0,
			cwd: null_mut(),
			names: HashMap::new(),
			nodes: HashMap::new(),
		};
		let root = Node::Directory(Directory::new(0, 0, Path::new()));

		fs.names.insert(root.abs_path().to_owned(), 0);
		fs.nodes.insert(0, root);

		// SAFETY: We just created this object
		let root = fs.nodes.get_mut(&0).unwrap();
		fs.cwd = root.as_dir_mut().unwrap() as *mut Directory;

		fs
	}

	fn cwd_as_ref(&self) -> &Directory {
		// SAFETY: `self.cwd` was set in `new` and `cd`, it will never be null.
		unsafe { self.cwd.as_ref().expect("Oops, cwd was null :(") }
	}

	fn cwd_as_mut(&mut self) -> &mut Directory {
		// SAFETY: `self.cwd` was set in `new` and `cd`, it will never be null.
		unsafe { self.cwd.as_mut().expect("Oops, cwd was null :(") }
	}

	fn cd(&mut self, name: &str) {
		if name == ".." {
			let mut path = self.cwd_as_ref().path.clone();
			path.parent().expect("At root?");
			let nid = self.names.get(path.abs_path()).expect("Path not parsed right?");
			let nwd =
				self.nodes.get_mut(nid).expect("Bad INode cd?").as_dir_mut().expect("isn't dir");
			self.cwd = nwd as *mut Directory;
		} else {
			if name.starts_with("/") {
				// Handle a case where the name is an abs_path
				let nid = self.names.get(name).expect("Bad name");
				let nwd =
					self.nodes.get_mut(nid).expect("Bad INode").as_dir_mut().expect("isn't dir");
				self.cwd = nwd as *mut Directory;
			} else {
				for inode in self.cwd_as_ref().children().iter().cloned() {
					let nwd = self.nodes.get(&inode).expect("Bad INode");
					if nwd.name() == name {
						let nwd =
							self.nodes.get_mut(&inode).unwrap().as_dir_mut().expect("isn't dir");
						self.cwd = nwd as *mut Directory;
						return;
					}
				}
			}
		}
	}

	fn allocate(&mut self, node: Node) {
		self.next_index += 1;
		let index = self.next_index;

		self.cwd_as_mut().add_as_child(index);

		self.names.insert(node.abs_path().to_owned(), index);
		self.nodes.insert(index, node);
	}

	fn new_file(&mut self, name: &str, size: usize) {
		// Update the size of all parent directories.
		let mut cpd = self.cwd_as_mut();
		loop {
			cpd.size += size;
			if cpd.parent_inode().is_none() {
				break;
			}
			let id = cpd.parent_inode().unwrap();
			cpd = self.nodes
				.get_mut(&id)
				.expect("Oops, no parent :(")
				.as_dir_mut()
				.expect("isn't directory");
		}

		let mut path = self.cwd_as_ref().path.clone();
		path.child(name).expect("The name had a slash in it :(");

		self.allocate(Node::File(File::new(self.next_index, size, path)))
	}

	fn new_dir(&mut self, name: &str) {
		let mut path = self.cwd_as_ref().path.clone();
		path.child(name).expect("The name had a slash in it :(");

		let parent = self.cwd_as_ref().inode;

		self.allocate(Node::Directory(Directory::new(self.next_index, 0, path).with_parent(parent)))
	}
}

impl<'a, I: Iterator<Item = Input<'a>>> From<I> for FileSystem {
	fn from(inputs: I) -> Self {
		let mut fs = Self::new();

		for ip in inputs {
			match ip {
				Input::Command(cmd) => match cmd {
					Command::List => continue,
					Command::Change(name) => fs.cd(name),
				},
				Input::Directory(name) => fs.new_dir(name),
				Input::File(size, name) => fs.new_file(name, size),
			}
		}

		fs
	}
}

#[test]
fn test_part1() {
	let input = get_input("day_7_test.txt");

	let fs = FileSystem::from(input.lines().map(|s| Input::try_from(s).expect("Bad Input")));
	println!("{}", fs.cwd_as_ref().size);
}

#[test]
fn part1() {}

#[test]
fn part2() {}
