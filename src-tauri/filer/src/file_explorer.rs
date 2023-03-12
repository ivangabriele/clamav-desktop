use serde::{Deserialize, Serialize};

use crate::{file_list, types, utils};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FileExplorerNode {
    pub children: FileExplorerTree,
    pub drive: String,
    pub index_path: Vec<usize>,
    pub is_checked: bool,
    pub is_expanded: bool,
    pub kind: types::FileKind,
    pub name: String,
    pub path: String,
    pub path_components: Vec<String>,
    pub depth: usize,
}

pub type FileExplorerTree = Vec<FileExplorerNode>;

// -----------------------------------------------------------------------------
// Implementations

pub struct FileExplorer {
    tree: FileExplorerTree,
}
impl FileExplorer {
    pub fn new(tree: FileExplorerTree) -> Self {
        Self { tree }
    }

    pub fn into_checked_paths(&self) -> Vec<String> {
        self.find_checked_nodes(None)
            .into_iter()
            .map(|node| node.path)
            .collect()
    }

    pub fn into_tree(&self) -> FileExplorerTree {
        self.tree.to_owned()
    }

    pub fn toggle_is_checked(&mut self, index_path: Vec<usize>) -> () {
        let targeted_node = self.find_node(index_path.clone(), None);

        if targeted_node.is_checked {
            self.uncheck_node(targeted_node);
        } else {
            self.check_node(targeted_node);
        }
    }

    pub fn toggle_is_expanded(&mut self, index_path: Vec<usize>) -> () {
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

    fn expand_node(&mut self, index_path: Vec<usize>) -> () {
        let current_node = self.find_node(index_path.clone(), None);
        let directory_absolute_path_option = Some(utils::normalize_path(
            current_node.path_components.join("/"),
        ));
        let next_children = file_list::list(
            false,
            directory_absolute_path_option,
            Some(types::FileKind::Directory),
        )
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

    fn find_checked_nodes(
        &self,
        in_node_option: Option<FileExplorerNode>,
    ) -> Vec<FileExplorerNode> {
        let nodes = match in_node_option {
            Some(node) => node.children,
            None => self.tree.to_owned(),
        };

        let checked_nodes: Vec<FileExplorerNode> = nodes
            .to_owned()
            .into_iter()
            .filter(|node| node.is_checked)
            .collect();

        let children_checked_nodes: Vec<FileExplorerNode> = nodes
            .to_owned()
            .into_iter()
            .filter(|node| !node.is_checked)
            .map(|node| self.find_checked_nodes(Some(node)))
            .flatten()
            .collect();

        return checked_nodes
            .into_iter()
            .chain(children_checked_nodes.into_iter())
            .collect();
    }

    fn find_node(
        &self,
        index_path: Vec<usize>,
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
        index_path: Vec<usize>,
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

    fn unexpand_node(&mut self, index_path: Vec<usize>) -> () {
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
