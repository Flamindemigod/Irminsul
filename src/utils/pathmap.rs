use normpath::PathExt;
use rayon::prelude::*;
use std::{
    collections::{BTreeSet, HashSet, VecDeque},
    ffi::OsStr,
    path::PathBuf,
};

use crate::modules::treemap::Treemap;

// fn path_map_inner(
//     mut files: Vec<VecDeque<&OsStr>>,
//     prev_path: PathBuf>,
// ) -> BTreeSet<Box<Treemap>> {
//     let path_spilt: Vec<_> = files
//         .par_iter_mut()
//         .filter(|f| !f.is_empty())
//         .map(|f| (PathBuf::from(f.pop_front().unwrap()), f))
//         .collect();
//     let roots: HashSet<_> = path_spilt.par_iter().map(|split| split.0.clone()).collect();
//     let treemaps: Vec<_> = roots
//         .par_iter()
//         .map(|root| {
//             let paths: Vec<_> = path_spilt
//                 .par_iter()
//                 .filter(|path| path.0.eq(root))
//                 .map(|path| path.1.clone())
//                 .collect();
//             let mut branches: BTreeSet<Box<Treemap>> = path_map_inner(paths);
//             let mut node = root.clone();
//             let mut depth = 0;
//             if branches.len() == 1 {
//                 let branch = branches.first().unwrap();
//                 node = node.join(branch.node);
//                 branches = branch.branches;
//                 depth = branch.depth + 1;
//             }
//             Box::new(Treemap::new(node, depth, branches, pre));
//         })
//         .collect();
//     treemaps
// }
pub fn path_map(_files: Vec<PathBuf>) -> Vec<Box<Treemap>> {
    // let bind: Vec<_> = files
    //     .par_iter()
    //     .filter_map(|f| {
    //         if f.exists() {
    //             Some(f.normalize().unwrap().as_path().to_path_buf())
    //         } else {
    //             None
    //         }
    //     })
    //     .collect();
    // let file_vec: Vec<VecDeque<_>> = bind
    //     .par_iter()
    //     .map(|path| path.iter().collect::<VecDeque<_>>())
    //     .collect();
    // path_map_inner(file_vec, PathBuf::new())
    //     .iter()
    //     .map(|f| f.to_owned())
    //     .collect::<Vec<_>>()
    Vec::new()
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::{env::temp_dir, fs};
//     #[test]
//     fn test_path_map_empty_input() {
//         let files = Vec::new();
//         assert_eq!(path_map(files), Vec::new());
//     }

//     #[test]
//     fn test_path_map_single_file() {
//         let dir = temp_dir();
//         let file_path = dir.join("file_path_map.txt");
//         fs::File::create(&file_path).unwrap();
//         let files = vec![file_path.clone()];
//         let com_path = path_map(files);
//         let mut result = Vec::new();
//         result.push(Box::new(Treemap::new(
//             file_path.clone(),
//             file_path.components().collect::<Vec<_>>().len() - 1,
//             Vec::new(),
//             PathBuf::new(),
//         )));
//         assert_eq!(com_path, result);
//         fs::remove_file(file_path).unwrap();
//     }

//     #[test]
//     fn test_path_map_multiple_files() {
//         let dir = temp_dir();
//         let mut file1_path = dir.join("file1.txt");
//         let file2_path = dir.join("file2.txt");
//         let file3_path = dir.join("file3.txt");
//         fs::File::create(&file1_path).unwrap();
//         fs::File::create(&file2_path).unwrap();
//         fs::File::create(&file3_path).unwrap();

//         let files = vec![file1_path.clone(), file2_path.clone()];
//         let mut com_path = path_map(files.clone());

//         file1_path.pop();
//         let mut result = Vec::new();
//         result.push(Box::new(Treemap::new(
//             file1_path.clone(),
//             dir.components().collect::<Vec<_>>().len() - 1,
//             Vec::from([
//                 Box::new(Treemap::new(
//                     PathBuf::from("file1.txt"),
//                     0,
//                     Vec::new(),
//                     file1_path.clone(),
//                 )),
//                 Box::new(Treemap::new(
//                     PathBuf::from("file2.txt"),
//                     0,
//                     Vec::new(),
//                     file1_path.clone(),
//                 )),
//             ]),
//             PathBuf::new(),
//         )));
//         assert_eq!(com_path, result);
//         com_path = path_map(files.clone());
//         result = Vec::new();
//         result.push(Box::new(Treemap::new(
//             file1_path.clone(),
//             dir.components().collect::<Vec<_>>().len() - 1,
//             Vec::from([
//                 Box::new(Treemap::new(
//                     PathBuf::from("file2.txt"),
//                     0,
//                     Vec::new(),
//                     file1_path.clone(),
//                 )),
//                 Box::new(Treemap::new(
//                     PathBuf::from("file3.txt"),
//                     0,
//                     Vec::new(),
//                     file1_path.clone(),
//                 )),
//             ]),
//             PathBuf::new(),
//         )));
//         assert_ne!(com_path, result);
//         fs::remove_file(&file1_path.join("file1.txt")).unwrap();
//         fs::remove_file(file2_path).unwrap();
//         fs::remove_file(file3_path).unwrap();
//     }
// }
