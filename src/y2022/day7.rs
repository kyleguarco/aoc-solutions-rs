use std::{collections::HashMap, fmt::Display};

use crate::get_input;

#[derive(Debug)]
enum Command<'a> {
	List,
	ChangeDirectory(&'a str),
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
			"cd" => Ok(Self::ChangeDirectory(s.next().ok_or(())?)),
			_ => Err(()),
		}
	}
}

#[derive(Debug)]
enum Input<'a> {
	Command(Command<'a>),
	NewDirectory(&'a str),
	NewFile(usize, &'a str),
}

impl<'a> TryFrom<&'a str> for Input<'a> {
	type Error = ();

	fn try_from(s: &'a str) -> Result<Self, Self::Error> {
		let mut words = s.split_whitespace();
		let start = words.next().ok_or(())?;

		match start {
			"$" => Ok(Self::Command(s.try_into()?)),
			"dir" => Ok(Self::NewDirectory(words.next().ok_or(())?)),
			// Handle the case where the first word is a file size
			_ => {
				let size = start.parse::<usize>().map_err(|_| ())?;
				let name = words.next().ok_or(())?;
				Ok(Self::NewFile(size, name))
			}
		}
	}
}

#[derive(Clone, Debug)]
enum IOError {
	NameContainsSlash,
	NotADirectory,
	IsRoot,
	PathDoesNotExist,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Path {
	pstr: String,
}

impl Path {
	fn new() -> Self {
		Self {
			pstr: String::from("/"),
		}
	}

	fn child(mut self, name: &str) -> Result<Self, IOError> {
		if name.contains("/") {
			return Err(IOError::NameContainsSlash);
		}

		self.pstr.push_str(name);
		self.pstr.push('/');

		Ok(self)
	}
}

impl From<&str> for Path {
	fn from(value: &str) -> Self {
		Self {
			pstr: String::from(value),
		}
	}
}

impl Display for Path {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "\"{}\"", self.pstr)
	}
}

type NodeID = usize;

#[derive(Clone, Debug)]
struct ChildDirectory {
	inode: NodeID,
	parent: NodeID,
	size: usize,
	path: Path,
	children: Vec<NodeID>,
}

impl ChildDirectory {
	fn new(inode: NodeID, parent: NodeID, size: usize, path: Path) -> Self {
		Self {
			inode,
			parent,
			size,
			path,
			children: Vec::new(),
		}
	}

	fn parent_inode(&self) -> NodeID {
		self.parent
	}
}

#[derive(Clone, Debug)]
struct RootDirectory {
	size: usize,
	path: Path,
	children: Vec<NodeID>,
}

impl RootDirectory {
	fn new() -> Self {
		Self {
			size: 0,
			path: Path::new(),
			children: Vec::new(),
		}
	}

	const fn inode(&self) -> NodeID {
		0
	}

	fn path(&self) -> &Path {
		&self.path
	}
}

#[derive(Clone, Debug)]
enum Node {
	// It's constructed like this to save lines; We never reference File.
	File((NodeID, usize, Path)),
	Directory(ChildDirectory),
	Root(RootDirectory),
}

impl Node {
	fn add_as_child(&mut self, child: NodeID) -> Result<(), IOError> {
		let children: &mut Vec<NodeID> = match self {
			Self::Root(root) => &mut root.children,
			Self::Directory(dir) => &mut dir.children,
			Self::File(_) => return Err(IOError::NotADirectory),
		};

		children.push(child);

		Ok(())
	}

	fn upsize(&mut self, upsize: usize) {
		let size: &mut usize = match self {
			Self::File(file) => &mut file.1,
			Self::Directory(dir) => &mut dir.size,
			Self::Root(root) => &mut root.size,
		};

		*size += upsize;
	}

	fn inode(&self) -> NodeID {
		match self {
			Self::File(file) => file.0,
			Self::Directory(dir) => dir.inode,
			Self::Root(root) => root.inode(),
		}
	}

	fn path(&self) -> &Path {
		match self {
			Self::File(file) => &file.2,
			Self::Directory(dir) => &dir.path,
			Self::Root(root) => &root.path(),
		}
	}

	fn size(&self) -> usize {
		match self {
			Self::File(file) => file.1,
			Self::Directory(dir) => dir.size,
			Self::Root(root) => root.size,
		}
	}

	fn is_dir(&self) -> bool {
		match self {
			Self::Directory(_) => true,
			_ => false,
		}
	}

	fn as_dir(&self) -> Option<&ChildDirectory> {
		match self {
			Self::File(_) => None,
			Self::Directory(dir) => Some(dir),
			Self::Root(_) => None,
		}
	}
}

#[derive(Debug)]
struct FileSystem {
	next_index: NodeID,
	cwd: NodeID,
	names: HashMap<Path, NodeID>,
	nodes: HashMap<NodeID, Node>,
}

impl FileSystem {
	fn new() -> Self {
		let mut names = HashMap::new();
		let mut nodes = HashMap::new();

		// Pre-allocate root before other directories.
		let root = RootDirectory::new();
		names.insert(root.path().clone(), root.inode());
		nodes.insert(root.inode(), Node::Root(root));

		Self {
			next_index: 1,
			cwd: 0,
			names,
			nodes,
		}
	}

	fn cwd_as_ref(&self) -> &Node {
		// SAFETY: We know this node exists. We ensure its existence in self.cd
		self.nodes.get(&self.cwd).unwrap()
	}

	fn cwd_as_mut(&mut self) -> &mut Node {
		// SAFETY: We know this node exists. We ensure its existence in self.cd
		self.nodes.get_mut(&self.cwd).unwrap()
	}

	fn nodes(&self) -> impl Iterator<Item = &Node> {
		self.nodes.values()
	}

	fn derive_path(&self, name: &str) -> Result<Path, IOError> {
		self.cwd_as_ref().path().clone().child(name)
	}

	fn allocate(&mut self, node: Node) -> Result<(), IOError> {
		let index = self.next_index;

		self.cwd_as_mut().add_as_child(index)?;

		self.names.insert(node.path().clone(), index);
		self.nodes.insert(index, node);

		self.next_index += 1;

		Ok(())
	}

	fn new_file(&mut self, name: &str, size: usize) -> Result<(), IOError> {
		let path = self.derive_path(name)?;

		self.allocate(Node::File((self.next_index, size, path)))?;

		// Update the size of all parent directories.
		let mut cpd = self.cwd_as_mut();
		while cpd.is_dir() {
			cpd.upsize(size);

			// SAFETY: We know the current iteration of `cpd` is a directory.
			let id = cpd.as_dir().unwrap().parent_inode();
			cpd = self.nodes.get_mut(&id).unwrap();
		}
		// The final iteration asserts that `cpd` is the root directory.
		cpd.upsize(size);

		Ok(())
	}

	fn new_dir(&mut self, name: &str) -> Result<(), IOError> {
		let path = self.derive_path(name)?;

		let parent = self.cwd_as_ref().inode();

		self.allocate(Node::Directory(ChildDirectory::new(
			self.next_index,
			parent,
			0,
			path,
		)))?;

		Ok(())
	}

	fn cd(&mut self, name: &str) -> Result<(), IOError> {
		let nid = if name == ".." {
			self.cwd_as_ref()
				.as_dir()
				.ok_or(IOError::IsRoot)?
				.parent_inode()
		} else if name.starts_with("/") {
			*self
				.names
				.get(&Path::from(name))
				.ok_or(IOError::PathDoesNotExist)?
		} else {
			let path = self.derive_path(name)?;
			*self.names.get(&path).ok_or(IOError::PathDoesNotExist)?
		};

		self.cwd = nid;

		Ok(())
	}
}

// Wrapper struct to avoid issues with specialization.
struct FSInput<I>(I);

impl<'a, I: Iterator<Item = Input<'a>>> TryFrom<FSInput<I>> for FileSystem {
	type Error = IOError;

	fn try_from(inputs: FSInput<I>) -> Result<Self, Self::Error> {
		let mut fs = Self::new();

		for ip in inputs.0 {
			match ip {
				Input::Command(cmd) => match cmd {
					Command::List => continue,
					Command::ChangeDirectory(name) => fs.cd(name)?,
				},
				Input::NewDirectory(name) => fs.new_dir(name)?,
				Input::NewFile(size, name) => fs.new_file(name, size)?,
			}
		}

		Ok(fs)
	}
}

#[test]
fn part1() {
	let input = get_input("day_7.txt");

	let mut fs = FileSystem::try_from(FSInput(
		input
			.lines()
			.map(|s| Input::try_from(s).expect("Bad Input")),
	))
	.unwrap();

	fs.cd("/").unwrap();
	println!("Root Size: {}", fs.cwd_as_ref().size());

	let sum = fs
		.nodes()
		.filter_map(|n| {
			if n.is_dir() && n.size() <= 100000 {
				Some(n.size())
			} else {
				None
			}
		})
		.sum::<usize>();
	println!("Directory summation: {sum}");
}

#[test]
fn part2() {}
