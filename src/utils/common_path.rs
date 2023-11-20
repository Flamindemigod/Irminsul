use std::{
    collections::{BTreeSet, VecDeque},
    path::PathBuf,
};

use rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};

pub fn get_common_path(paths: Vec<PathBuf>) -> Option<PathBuf> {
    let mut com_path = PathBuf::new();
    let mut path_vecs: Vec<_> = paths
        .par_iter()
        .map(|path| {
            path.iter()
                .map(|f| f.to_str().unwrap())
                .collect::<VecDeque<_>>()
        })
        .collect();
    loop {
        let set: BTreeSet<_> = path_vecs
            .par_iter_mut()
            .filter_map(|v| v.pop_front())
            .collect();
        match set.len() {
            1 => com_path.push(set.first().unwrap()),
            _ => break,
        }
    }
    if com_path == PathBuf::new() {
        return None;
    }
    Some(com_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_com_path_empty_input() {
        let files = Vec::new();
        assert_eq!(get_common_path(files), None);
    }

    #[test]
    fn test_com_path_single_file() {
        // Why Yes, Bob Ross was indeed a linux user. And yes he used GIMP.
        let files = vec!["/home/bob_ross/paintings/working/the old mill/version6969420.xcf"]
            .iter()
            .map(|p| PathBuf::from(p))
            .collect();
        let com_path = get_common_path(files);
        assert_eq!(
            com_path,
            Some(PathBuf::from(
                "/home/bob_ross/paintings/working/the old mill/version6969420.xcf"
            ))
        );
    }

    #[test]
    fn test_com_path_single_file_repeated() {
        let files = vec![
            "/home/bob_ross/paintings/working/the old mill/version6969420.xcf",
            "/home/bob_ross/paintings/working/the old mill/version6969420.xcf",
        ]
        .iter()
        .map(|p| PathBuf::from(p))
        .collect();
        let com_path = get_common_path(files);
        assert_eq!(
            com_path,
            Some(PathBuf::from(
                "/home/bob_ross/paintings/working/the old mill/version6969420.xcf"
            ))
        );
    }

    #[test]
    fn test_com_path_multiple_files() {
        let files = vec![
            "/home/bob_ross/paintings/working/the old mill/version6969420.xcf",
            "/home/bob_ross/paintings/working/the old mill/version6942069.xcf",
        ]
        .iter()
        .map(|p| PathBuf::from(p))
        .collect();
        let com_path = get_common_path(files);
        assert_eq!(
            com_path,
            Some(PathBuf::from(
                "/home/bob_ross/paintings/working/the old mill"
            ))
        );
    }
    #[test]
    fn test_com_path_empty_path() {
        let files = vec![""].iter().map(|p| PathBuf::from(p)).collect();
        let com_path = get_common_path(files);
        assert_eq!(com_path, None);
    }
}
