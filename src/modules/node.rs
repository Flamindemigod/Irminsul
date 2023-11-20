use glob::glob;
use log::error;
use normpath::PathExt;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, path::PathBuf};

use crate::utils::get_common_path;

use super::treemap::Treemap;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Node {
    root: PathBuf,
    path_pattern: Option<Box<str>>,
    exec: Option<String>,
}

impl Node {
    #[cfg(target_family = "unix")]
    fn build_paths(&self) -> Vec<PathBuf> {
        if self.path_pattern.is_some() {
            match glob(
                self.root
                    .join(
                        self.path_pattern
                            .clone()
                            .expect("Path Pattern is Missing")
                            .as_ref(),
                    )
                    .to_str()
                    .expect("Path Pattern And/Or Root Does not Contain any Chars"),
            ) {
                Err(err) => {
                    error!("Failed to read Glob Pattern: {err}");
                    Vec::new()
                }
                Ok(paths) => {
                    return paths
                        .filter_map(|path| {
                            match path
                                .map_err(|err| error!("Glob Pattern failed to resolve path: {err}"))
                            {
                                Err(_) => None,
                                Ok(p) => Some(
                                    p.normalize()
                                        .expect("Failed to Normalize Path")
                                        .into_path_buf(),
                                ),
                            }
                        })
                        .collect()
                }
            }
        } else {
            vec![self.root.clone()]
        }
    }

    #[cfg(target_family = "windows")]
    fn build_paths(&self) -> Vec<PathBuf> {
        if self.path_pattern.is_some() {
            match glob(
                self.root
                    .join(
                        self.path_pattern
                            .clone()
                            .expect("Path Pattern is Missing")
                            .as_ref(),
                    )
                    .to_str()
                    .expect("Path Pattern And/Or Root Does not Contain any Chars"),
            ) {
                Err(err) => {
                    error!("Failed to read Glob Pattern: {err}");
                    Vec::new()
                }
                Ok(paths) => {
                    return paths
                        .filter_map(|path| {
                            match path
                                .map_err(|err| error!("Glob Pattern failed to resolve path: {err}"))
                            {
                                Err(_) => None,
                                Ok(p) => Some(
                                    PathBuf::from("WinRoot").join(
                                        p.normalize()
                                            .expect("Failed to Normalize Path")
                                            .into_path_buf(),
                                    ),
                                ),
                            }
                        })
                        .collect()
                }
            }
        } else {
            vec![PathBuf::from("WinRoot").join(self.root.clone())]
        }
    }

    fn build_treemap(&self) -> Treemap {
        let paths = self.build_paths();
        Treemap::new(PathBuf::new(), 0, BTreeSet::new(), PathBuf::new())
    }
}
