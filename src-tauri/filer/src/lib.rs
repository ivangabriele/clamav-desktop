// TODO Handle permission errors
// https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html#see-also-5

use std::{fmt::Display, path::PathBuf};

use glob::glob;
use serde::{Deserialize, Serialize};

pub mod drive;

// -----------------------------------------------------------------------------
// Types

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum FileKind {
    Directory,
    File,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FileExplorerNode {
    pub children: FileExplorerTree,
    pub index_path: IndexPath,
    pub is_checked: bool,
    pub is_expanded: bool,
    pub kind: FileKind,
    pub path: Vec<String>,
}

pub type FileExplorerTree = Vec<FileExplorerNode>;
pub type IndexPath = Vec<usize>;
pub type PathbufList = Vec<PathBuf>;

// -----------------------------------------------------------------------------
// Implementations

pub struct FileExplorer {
    tree: FileExplorerTree,
}
impl FileExplorer {
    pub fn new(tree: FileExplorerTree) -> Self {
        Self { tree }
    }

    pub fn into_tree(&self) -> FileExplorerTree {
        self.tree.to_owned()
    }

    pub fn toggle_is_checked(&mut self, index_path: IndexPath) -> () {
        let targeted_node = self.find_node(index_path.clone(), None);

        if targeted_node.is_checked {
            self.uncheck_node(targeted_node);
        } else {
            self.check_node(targeted_node);
        }
    }

    pub fn toggle_is_expanded(&mut self, index_path: IndexPath) -> () {
        let targeted_node = self.find_node(index_path.clone(), None);

        if targeted_node.is_expanded {
            self.unexpand_node(index_path);
        } else {
            self.expand_node(index_path);
        }
    }

    pub fn check_node(&mut self, targeted_node: FileExplorerNode) -> () {
        let updated_node = FileExplorerNode {
            children: vec![],
            is_checked: true,
            ..targeted_node.to_owned()
        };
        let updated_tree =
            self.replace_node(targeted_node.to_owned().index_path, updated_node, None);

        self.tree = updated_tree;
    }

    fn expand_node(&mut self, index_path: IndexPath) -> () {
        let current_node = self.find_node(index_path.clone(), None);
        let path_as_string = normalize_path(current_node.path.join("/"));
        let next_children = list(path_as_string, false, Some(FileKind::Directory))
            .into_file_explorer()
            .into_tree()
            .into_iter()
            .map(|node| FileExplorerNode {
                index_path: current_node
                    .index_path
                    .to_owned()
                    .into_iter()
                    .chain(node.index_path.into_iter())
                    .collect(),
                ..node
            })
            .collect();
        let updated_node = FileExplorerNode {
            children: next_children,
            is_expanded: true,
            ..current_node
        };
        let updated_tree = self.replace_node(index_path.clone(), updated_node, None);

        self.tree = updated_tree;
    }

    fn find_node(
        &self,
        index_path: IndexPath,
        in_node_option: Option<FileExplorerNode>,
    ) -> FileExplorerNode {
        let index = index_path.get(0).unwrap().to_owned();

        let next_node = match in_node_option {
            Some(node) => node.children.get(index).unwrap().to_owned(),
            None => self.tree.get(index).unwrap().to_owned(),
        };

        if index_path.len() == 1 {
            return next_node;
        }

        let next_index_path = index_path.get(1..).unwrap().to_owned();

        self.find_node(next_index_path, Some(next_node))
    }

    fn replace_node(
        &self,
        index_path: IndexPath,
        updated_node: FileExplorerNode,
        in_tree_option: Option<FileExplorerTree>,
    ) -> FileExplorerTree {
        let index = index_path.get(0).unwrap().to_owned();
        let current_tree = in_tree_option.unwrap_or(self.tree.to_owned());

        current_tree
            .iter()
            .enumerate()
            .map(|(node_index, node)| {
                let node_clone = node.clone();

                if node_index == index {
                    let updated_node_owned = updated_node.to_owned();

                    if index_path.len() == 1 {
                        return updated_node_owned;
                    }

                    let next_index_path = index_path.get(1..).unwrap().to_owned();

                    return FileExplorerNode {
                        children: self.replace_node(
                            next_index_path,
                            updated_node_owned,
                            Some(node.children.to_owned()),
                        ),
                        ..node_clone
                    };
                }

                node_clone
            })
            .collect()
    }

    pub fn uncheck_node(&mut self, targeted_node: FileExplorerNode) -> () {
        let updated_node = FileExplorerNode {
            children: vec![],
            is_checked: false,
            ..targeted_node.to_owned()
        };
        let updated_tree =
            self.replace_node(targeted_node.to_owned().index_path, updated_node, None);

        self.tree = updated_tree;
    }

    fn unexpand_node(&mut self, index_path: IndexPath) -> () {
        let current_node = self.find_node(index_path.clone(), None);
        let updated_node = FileExplorerNode {
            children: vec![],
            is_expanded: false,
            ..current_node
        };
        let updated_tree = self.replace_node(index_path.clone(), updated_node, None);

        self.tree = updated_tree;
    }
}

/// Represents a file list.
pub struct FileList {
    path_bufs: Vec<PathBuf>,
}
impl FileList {
    pub fn new(path_bufs: Vec<PathBuf>) -> Self {
        Self { path_bufs }
    }

    pub fn into_file_explorer(&self) -> FileExplorer {
        let tree: FileExplorerTree = self
            .path_bufs
            .to_owned()
            .into_iter()
            .enumerate()
            .filter_map(|(index, path_buf)| -> Option<FileExplorerNode> {
                Some(FileExplorerNode {
                    index_path: vec![index],
                    children: FileExplorerTree::new(),
                    path: path_buf
                        .components()
                        .map(|component| match component.as_os_str().to_str() {
                            Some(component_as_str) => component_as_str.to_string(),
                            // TODO Handle this case errors here by returning None to `filter_map()`.
                            // https://stackoverflow.com/a/63120052/2736233
                            None => String::from(""),
                        })
                        .collect(),
                    is_checked: false,
                    is_expanded: false,
                    kind: self.get_file_kind_from_path_buf(path_buf),
                })
            })
            .collect();

        FileExplorer::new(tree)
    }

    pub fn into_strings(&self) -> Vec<String> {
        self.path_bufs
            .to_owned()
            .into_iter()
            .filter_map(|path_buf| match path_buf.as_os_str().to_str() {
                Some(path_as_str) => Some(path_as_str.to_string()),
                None => None,
            })
            .collect()
    }

    /// Returns a `FileKind` from a `PathBuf`.
    ///
    /// # Panics
    ///
    /// This method panics if `PathBuf` is neither a file or a directory. This should never happen.
    fn get_file_kind_from_path_buf(&self, path_buf: PathBuf) -> FileKind {
        if path_buf.is_dir() {
            return FileKind::Directory;
        }

        if path_buf.is_file() {
            return FileKind::File;
        }

        panic!(
            "<path_buf> must be a directory or a file, got {}. Is it a symbolic link?",
            path_buf.display()
        );
    }
}

// -----------------------------------------------------------------------------
// Functions

/// List all files within the provided directory path.
///
/// Always exclude (and doesn't follow) symbolic links.
///
/// # Examples
///
/// ```no_run
/// use filer;
///
/// assert_eq!(filer::list("/".to_string(), false, Some(filer::FileKind::Directory)).into_strings()[0], "/bin");
/// ```
// We add a `` ```no run `` here because it's too platform-specific to be consistently tested.
// TODO Use `Path` instead of `String` for <directory_absolute_path>?
// TODO Check for <directory_absolute_path> existence and type.
#[allow(dead_code)]
pub fn list<S>(
    directory_absolute_path: S,
    is_recursive: bool,
    file_kind_option: Option<FileKind>,
) -> FileList
where
    S: AsRef<str> + Display,
{
    let pattern_suffix = match is_recursive {
        true => "/**/*",
        false => "/*",
    };
    let pattern = normalize_path(format!("{}{}", directory_absolute_path, pattern_suffix));
    let pattern_as_str = &*pattern;
    // https://github.com/rust-lang/glob is kind of a dead or dormant repository.
    // It doesn't give the option to avoid following synbolic links which may heavily impact this function performance
    // and forces us to deduplicate path strings.
    match glob(pattern_as_str) {
        Ok(paths) => {
            let path_bufs: Vec<PathBuf> = paths
                .filter_map(|path_buf_result| path_buf_result.ok())
                .collect();

            let symlink_directory_paths: Vec<String> = path_bufs
                .to_owned()
                .into_iter()
                .filter_map(|path_buf| {
                    if path_buf.is_dir() && path_buf.is_symlink() {
                        return Some(path_buf);
                    }

                    None
                })
                .filter_map(|path_buf| match path_buf.as_os_str().to_str() {
                    Some(path_as_str) => Some(path_as_str.to_string()),
                    None => None,
                })
                .collect();

            let filtered_path_bufs: Vec<PathBuf> = path_bufs
                .to_owned()
                .into_iter()
                .filter(|path_buf| match file_kind_option {
                    Some(FileKind::Directory) => path_buf.is_dir() && !path_buf.is_symlink(),
                    Some(FileKind::File) => path_buf.is_file() && !path_buf.is_symlink(),
                    None => !path_buf.is_symlink(),
                })
                // TODO Replace that with a path sorting function used with `Vec.sort_unstable_by()` + `Vec.dedup()`.
                .filter(|path_buf| {
                    for symlink_directory_path in symlink_directory_paths.to_owned() {
                        if path_buf.starts_with(symlink_directory_path) {
                            return false;
                        }
                    }

                    true
                })
                .collect();

            // `Vec.dedup()` only removes consecutive repeated elements, so we first need to sort it.
            // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.dedup
            // `Vec.sort_unstable()` is faster than `Vec.sort()`.
            // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_unstable
            // path_bufs.sort_unstable();
            // path_bufs.dedup();

            FileList::new(filtered_path_bufs)
        }
        Err(..) => FileList { path_bufs: vec![] },
    }
}

pub fn normalize_path<S>(path: S) -> String
where
    S: AsRef<str>,
{
    let path_as_ref = path.as_ref();

    if cfg!(windows) {
        let normalized_path_as_string = path_as_ref.replace("/", "\\");

        return normalized_path_as_string;
    }

    path_as_ref.to_string()
}
