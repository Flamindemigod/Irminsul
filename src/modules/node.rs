use glob::glob;
use log::error;
use normpath::PathExt;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, path::PathBuf};

use super::treemap::Treemap;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Node {
    path_pattern: Box<str>,
    exec: String,
}

impl Node {
    fn build_paths(self) -> Vec<PathBuf> {
        match glob(self.path_pattern.as_ref()) {
            Err(err) => {
                error!("Failed to read Glob Pattern: {err}");
                Vec::new()
            }
            Ok(paths) => paths
                .filter_map(|path| {
                    match path.map_err(|err| error!("Glob Pattern failed to resolve path: {err}")) {
                        Err(_) => None,
                        Ok(p) => Some(
                            p.normalize()
                                .expect("Failed to Normalize Path")
                                .into_path_buf(),
                        ),
                    }
                })
                .collect(),
        }
    }
    fn build_treemap(self) -> (Box<Self>, Treemap) {
        let paths = self.build_paths();
        paths.par_iter().map(|p| {});
        (
            Box::new(self),
            Treemap::new(PathBuf::new(), 0, BTreeSet::new(), PathBuf::new()),
        )
    }
}
