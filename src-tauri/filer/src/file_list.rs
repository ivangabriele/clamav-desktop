// TODO Handle permission errors
// https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html#see-also-5

use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use glob::glob;

use crate::{drive, file_explorer, types, utils};

/// Represents a file list.
pub struct FileList {
    pathbufs: Vec<PathBuf>,
}
impl FileList {
    pub fn new(pathbufs: Vec<PathBuf>) -> Self {
        Self { pathbufs }
    }

    pub fn into_file_explorer(&self) -> file_explorer::FileExplorer {
        let tree: file_explorer::FileExplorerTree = self
            .pathbufs
            .to_owned()
            .into_iter()
            .enumerate()
            .filter_map(
                |(index, pathbuf)| -> Option<file_explorer::FileExplorerNode> {
                    let path_components: Vec<String> = pathbuf
                        .components()
                        .filter_map(|component| Some(component.as_os_str().to_str()?.to_string()))
                        .collect();

                    // TODO Check for number errors.
                    let depth = path_components.len() - 1;
                    let drive = common::ok_or_return_none!(path_components.get(0)).to_owned();
                    let name = common::ok_or_return_none!(path_components.last()).to_owned();
                    let path = utils::normalize_path(path_components.join("/"));

                    Some(file_explorer::FileExplorerNode {
                        index_path: vec![index],
                        children: file_explorer::FileExplorerTree::new(),
                        depth,
                        drive,
                        is_checked: false,
                        is_expanded: false,
                        kind: self.get_file_kind_from_pathbuf(pathbuf),
                        name,
                        path,
                        path_components,
                    })
                },
            )
            .collect();

        file_explorer::FileExplorer::new(tree)
    }

    pub fn into_strings(&self) -> Vec<String> {
        self.pathbufs
            .to_owned()
            .into_iter()
            .filter_map(|pathbuf| match pathbuf.as_os_str().to_str() {
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
    fn get_file_kind_from_pathbuf(&self, pathbuf: PathBuf) -> types::FileKind {
        if pathbuf.is_dir() {
            return types::FileKind::Directory;
        }

        if pathbuf.is_file() {
            return types::FileKind::File;
        }

        panic!(
            "<pathbuf> must be a directory or a file, got {}. Is it a symbolic link?",
            pathbuf.display()
        );
    }
}

/// List all files within the provided directory path.
///
/// Always exclude (and doesn't follow) symbolic links.
///
/// # Examples
///
/// ```no_run
/// use filer;
///
/// assert_eq!(filer::list(false, Some("/"), Some(filer::FileKind::Directory)).into_strings()[0], "/bin");
/// ```
// We add a `` ```no run `` here because it's too platform-specific to be consistently tested.
// TODO Use `Path` instead of `String` for <directory_absolute_path>?
// TODO Check for <directory_absolute_path> existence and type.
pub fn list<S>(
    is_recursive: bool,
    directory_absolute_path_option: Option<S>,
    file_kind_option: Option<types::FileKind>,
) -> FileList
where
    S: AsRef<str> + Display,
{
    let pathbufs: Vec<PathBuf> = match directory_absolute_path_option {
        None => drive::list()
            .into_iter()
            .map(|drive| Path::new(&*drive).to_owned())
            .collect(),
        Some(directory_absolute_path) => {
            let pattern_suffix = match is_recursive {
                true => "/**/*",
                false => "/*",
            };
            let pattern =
                utils::normalize_path(format!("{}{}", directory_absolute_path, pattern_suffix));

            // https://github.com/rust-lang/glob is kind of a dead or dormant repository.
            // It doesn't give the option to avoid following synbolic links which may heavily impact this function performance
            // and forces us to deduplicate path strings.
            match glob(&*pattern) {
                Ok(paths) => paths
                    .filter_map(|pathbuf_result| pathbuf_result.ok())
                    .collect(),
                Err(..) => vec![],
            }
        }
    };

    // Symbolic links are parsed as normal files and directories in Windows < 10?
    let symlink_directory_paths: Vec<String> = pathbufs
        .to_owned()
        .into_iter()
        .filter_map(|pathbuf| {
            if pathbuf.is_dir() && pathbuf.is_symlink() {
                return Some(pathbuf);
            }

            None
        })
        .filter_map(|pathbuf| match pathbuf.as_os_str().to_str() {
            Some(path_as_str) => Some(path_as_str.to_string()),
            None => None,
        })
        .collect();

    let filtered_pathbufs: Vec<PathBuf> = pathbufs
        .to_owned()
        .into_iter()
        // Filter by file kind, exluding symbolic links
        .filter(|pathbuf| match file_kind_option {
            Some(types::FileKind::Directory) => pathbuf.is_dir() && !pathbuf.is_symlink(),
            Some(types::FileKind::File) => pathbuf.is_file() && !pathbuf.is_symlink(),
            None => !pathbuf.is_symlink(),
        })
        // TODO Replace that with a path sorting function used with `Vec.sort_unstable_by()` + `Vec.dedup()`.
        // Since glob crate still parse symbolic link directories, we have to filter out their content,
        // which is expensive for large number of files
        .filter(|pathbuf| {
            for symlink_directory_path in symlink_directory_paths.to_owned() {
                if pathbuf.starts_with(symlink_directory_path) {
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
    // pathbufs.sort_unstable();
    // pathbufs.dedup();

    FileList::new(filtered_pathbufs)
}

pub fn count<S>(
    is_recursive: bool,
    directory_absolute_path_option: Option<S>,
    file_kind_option: Option<types::FileKind>,
) -> usize
where
    S: AsRef<str> + Display,
{
    let file_paths = list(
        is_recursive,
        directory_absolute_path_option,
        file_kind_option,
    )
    .into_strings();

    file_paths.len()
}
