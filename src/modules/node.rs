use normpath::PathExt;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, ffi::OsStr, path::PathBuf};

use super::treemap::Treemap;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Node {
    root: PathBuf,
    path_pattern: Option<String>,
    exec: Option<String>,
}

impl Node {
    #[cfg(target_family = "unix")]
    fn build_paths(&self) -> Vec<PathBuf> {
        if self.path_pattern.is_some() {
            globmatch::Builder::new(
                self.path_pattern
                    .clone()
                    .expect("Path Pattern is Missing")
                    .as_str(),
            )
            .build(self.root.clone())
            .map_err(|err| eprintln!("Failed to Build Glob Pattern: {err}"))
            .unwrap()
            .into_iter()
            .flatten()
            .map(|f| {
                f.normalize()
                    .expect("Failed to Normalize Path")
                    .into_path_buf()
            })
            .collect()
        } else {
            vec![self.root.clone()]
        }
    }

    #[cfg(target_family = "windows")]
    fn build_paths(&self) -> Vec<PathBuf> {
        if self.path_pattern.is_some() {
            globmatch::Builder::new(
                self.path_pattern
                    .clone()
                    .expect("Path Pattern is Missing")
                    .as_str(),
            )
            .build(self.root.clone())
            .map_err(|err| eprintln!("Failed to Build Glob Pattern: {err}"))
            .unwrap()
            .into_iter()
            .flatten()
            .map(|f| {
                PathBuf::from("WinRoot").join(
                    f.normalize()
                        .expect("Failed to Normalize Path")
                        .into_path_buf(),
                )
            })
            .collect()
        } else {
            vec![PathBuf::from("WinRoot").join(self.root.clone())]
        }
    }

    fn build_treemap_inner(
        &self,
        path_segments: &mut VecDeque<&OsStr>,
        prev_segment: PathBuf,
    ) -> Box<Treemap> {
        let segment = path_segments
            .pop_front()
            .expect("Failed to Pop Front Element");
        let mut set = Vec::new();
        if !path_segments.is_empty() {
            let branch = self.build_treemap_inner(path_segments, prev_segment.join(segment));
            set.push(branch);
        }
        let node = Box::from(Treemap::new(segment.into(), 0, set, prev_segment));
        return node;
    }
    fn build_treemap(&self) -> Box<Treemap> {
        let paths = self.build_paths();
        println!("{:#?}", paths);
        let mut maps = paths
            .par_iter()
            .map(|path| {
                let mut path_segs = path.iter().collect::<VecDeque<_>>();
                self.build_treemap_inner(&mut path_segs, PathBuf::new())
                // path_segs.iter().map(|seg| Treemap::new(seg.into(), 0, branches, prev_path))
            })
            .collect::<Vec<_>>();
        let parent = maps.pop().expect("Must have atleast one path");

        let mut parent = parent.as_ref().clone();
        while !maps.is_empty() {
            let other = maps.pop().unwrap();
            parent.merge(&mut other.as_ref().clone());
        }
        Box::from(parent)
    }
}

#[cfg(test)]
mod tests {
    use log::error;
    use pretty_assertions::{assert_eq, assert_ne};
    use std::{env::temp_dir, fs};

    use super::*;
    #[test]
    fn test_conf_node_one_file() {
        let conf_node = Node {
            exec: None,
            root: PathBuf::from("/home/bob_ross"),
            path_pattern: None,
        };
        let mut branch_inner = Vec::new();
        branch_inner.push(Box::from(Treemap::new(
            PathBuf::from("bob_ross"),
            0,
            Vec::new(),
            PathBuf::from("/home"),
        )));
        let mut branch = Vec::new();
        branch.push(Box::from(Treemap::new(
            PathBuf::from("home"),
            0,
            branch_inner,
            PathBuf::from("/"),
        )));
        assert_eq!(
            conf_node.build_treemap(),
            Box::from(Treemap::new(PathBuf::from("/"), 0, branch, PathBuf::new()))
        );
    }

    #[test]
    fn test_conf_node_multiple_file() {
        let temp_dir = temp_dir().join("bob_ross1");
        let _ =
            fs::create_dir_all(&temp_dir).map_err(|err| error!("Failed to Create Temp Dir {err}"));
        let file1_path = temp_dir.join("the old mill.png");
        let file2_path = temp_dir.join("mountain retreat.png");
        let _ = fs::File::create(&file1_path)
            .map_err(|err| error!("Failed to Create Temp File {}: {err}", file1_path.display()));
        let _ = fs::File::create(&file2_path)
            .map_err(|err| error!("Failed to Create Temp File {}: {err}", file2_path.display()));

        let mut temp_treemap = Node {
            root: temp_dir.clone(),
            path_pattern: None,
            exec: None,
        }
        .build_treemap();
        let mut branch_inner = Vec::new();
        branch_inner.push(Box::from(Treemap::new(
            PathBuf::from("the old mill.png"),
            0,
            Vec::new(),
            temp_dir.clone(),
        )));
        branch_inner.push(Box::from(Treemap::new(
            PathBuf::from("mountain retreat.png"),
            0,
            Vec::new(),
            temp_dir.clone(),
        )));
        temp_treemap.branches[0].branches[0].branches = branch_inner;

        let conf_node = Node {
            exec: None,
            root: temp_dir.clone(),
            path_pattern: Some(String::from("*.png")),
        }
        .build_treemap();

        assert_eq!(conf_node, temp_treemap);
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_conf_node_multiple_files_one_nested_no_match() {
        let temp_dir = temp_dir().join("bob_ross2");
        let _ = fs::create_dir_all(&temp_dir.join("WIP"))
            .map_err(|err| error!("Failed to Create Temp Dir {err}"));
        let file1_path = temp_dir.join("the old mill.png");
        let file2_path = temp_dir.join("mountain retreat.png");
        let file3_path = temp_dir.join("WIP").join("Wilderness Day.png");
        let _ = fs::File::create(&file1_path)
            .map_err(|err| error!("Failed to Create Temp File {}: {err}", file1_path.display()));
        let _ = fs::File::create(&file2_path)
            .map_err(|err| error!("Failed to Create Temp File {}: {err}", file2_path.display()));
        let _ = fs::File::create(&file3_path)
            .map_err(|err| error!("Failed to Create Temp File {}: {err}", file3_path.display()));

        let mut temp_treemap = Node {
            root: temp_dir.clone(),
            path_pattern: None,
            exec: None,
        }
        .build_treemap();
        let mut branch_inner = Vec::new();
        branch_inner.push(Box::from(Treemap::new(
            PathBuf::from("the old mill.png"),
            0,
            Vec::new(),
            temp_dir.clone(),
        )));
        branch_inner.push(Box::from(Treemap::new(
            PathBuf::from("mountain retreat.png"),
            0,
            Vec::new(),
            temp_dir.clone(),
        )));
        branch_inner.push(Box::from(Treemap::new(
            PathBuf::from("WIP"),
            0,
            vec![Box::from(Treemap::new(
                PathBuf::from("Wilderness Day.png"),
                0,
                Vec::new(),
                temp_dir.clone().join("WIP"),
            ))],
            temp_dir.clone(),
        )));
        temp_treemap.branches[0].branches[0].branches = branch_inner;

        {
            let conf_node = Node {
                exec: None,
                root: temp_dir.clone(),
                path_pattern: Some(String::from("*.png")),
            }
            .build_treemap();

            assert_ne!(conf_node, temp_treemap);
        }

        {
            let conf_node = Node {
                exec: None,
                root: temp_dir.clone(),
                path_pattern: Some(String::from("**/*.png")),
            }
            .build_treemap();

            assert_eq!(conf_node, temp_treemap);
        }
        let _ = fs::remove_dir_all(temp_dir);
    }
}
