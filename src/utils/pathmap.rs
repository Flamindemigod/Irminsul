use normpath::PathExt;
use rayon::prelude::*;
use std::{
    collections::{HashSet, VecDeque},
    ffi::OsStr,
    path::PathBuf,
};

use super::treemap::Treemap;

fn path_map_inner(mut files: Vec<VecDeque<&OsStr>>) -> Vec<Box<Treemap<PathBuf>>> {
    let path_spilt: Vec<_> = files
        .par_iter_mut()
        .filter(|f| !f.is_empty())
        .map(|f| (PathBuf::from(f.pop_front().unwrap()), f))
        .collect();
    let roots: HashSet<_> = path_spilt.par_iter().map(|split| split.0.clone()).collect();
    let treemaps: Vec<_> = roots
        .par_iter()
        .map(|root| {
            let paths: Vec<_> = path_spilt
                .par_iter()
                .filter(|path| path.0.eq(root))
                .map(|path| path.1.clone())
                .collect();
            let mut branches: Vec<_> = path_map_inner(paths);
            let mut node = root.clone();
            let mut depth = 0;
            if branches.len() == 1 {
                let branch = branches.pop().unwrap();
                node = node.join(branch.node);
                branches = branch.branches;
                depth = branch.depth + 1;
            }
            Box::new(Treemap {
                node,
                branches,
                depth,
            })
        })
        .collect();
    treemaps
}
pub fn path_map(files: Vec<PathBuf>) -> Vec<Box<Treemap<PathBuf>>> {
    let bind: Vec<_> = files
        .par_iter()
        .map(|f| f.normalize().unwrap().as_path().to_path_buf())
        .collect();
    let file_vec: Vec<VecDeque<_>> = bind
        .par_iter()
        .map(|path| path.iter().collect::<VecDeque<_>>())
        .collect();
    path_map_inner(file_vec)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env::temp_dir, fs};

    #[test]
    fn test_path_map_empty_input() {
        let files = Vec::new();
        assert_eq!(path_map(files), []);
    }

    #[test]
    fn test_path_map_single_file() {
        let dir = temp_dir();
        let file_path = dir.join("file.txt");
        fs::File::create(&file_path).unwrap();

        let files = vec![file_path.clone()];
        let com_path = path_map(files);
        fs::remove_file(&file_path).unwrap();
        assert_eq!(
            com_path,
            vec![Box::new(Treemap {
                depth: file_path.components().collect::<Vec<_>>().len() - 1,
                node: file_path,
                branches: Vec::new()
            })]
        );
    }

    #[test]
    fn test_path_map_multiple_files() {
        let dir = temp_dir();
        let file1_path = dir.join("file1.txt");
        let file2_path = dir.join("file2.txt");
        fs::File::create(&file1_path).unwrap();
        fs::File::create(&file2_path).unwrap();

        let files = vec![file1_path.clone(), file2_path.clone()];
        let com_path = path_map(files);
        fs::remove_file(file1_path).unwrap();
        fs::remove_file(file2_path).unwrap();
        assert_eq!(
            com_path,
            vec![Box::new(Treemap {
                depth: dir.components().collect::<Vec<_>>().len() - 1,
                node: dir,
                branches: vec![
                    Box::new(Treemap {
                        node: PathBuf::from("file1.txt"),
                        depth: 0,
                        branches: Vec::new()
                    }),
                    Box::new(Treemap {
                        node: PathBuf::from("file2.txt"),
                        depth: 0,
                        branches: Vec::new()
                    })
                ]
            })]
        );
    }
}
