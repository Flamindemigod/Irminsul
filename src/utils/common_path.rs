use normpath::PathExt;
use rayon::prelude::*;
use std::{collections::VecDeque, path::PathBuf};

pub fn common_path(files: Vec<PathBuf>) -> Option<PathBuf> {
    let bind: Vec<_> = files
        .par_iter()
        .map(|f| f.normalize().unwrap().as_path().to_path_buf())
        .collect();
    let mut file_vec: Vec<VecDeque<_>> = bind
        .par_iter()
        .map(|path| path.iter().collect::<VecDeque<_>>())
        .collect();
    let mut common_path = PathBuf::new();
    loop {
        let front = file_vec
            .iter_mut()
            .map(|path| path.pop_front().unwrap_or_default())
            .fold(Option::None, |acc, val| {
                if !val.is_empty() {
                    if acc.is_none() || val.eq(acc.unwrap()) {
                        Some(val)
                    } else {
                        None
                    }
                } else {
                    None
                }
            });
        if front.is_none() {
            break;
        } else {
            common_path.push(front.unwrap())
        }
    }
    if common_path.eq(&PathBuf::new()) {
        return None;
    }
    Some(common_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env::temp_dir, fs};

    #[test]
    fn test_common_path_empty_input() {
        let files = Vec::new();
        assert_eq!(common_path(files), None);
    }

    #[test]
    fn test_common_path_single_file() {
        let dir = temp_dir();
        let file_path = dir.join("file.txt");
        fs::File::create(&file_path).unwrap();

        let files = vec![file_path.clone()];
        let com_path = common_path(files);
        fs::remove_file(&file_path).unwrap();
        assert_eq!(com_path, Some(file_path));
    }

    #[test]
    fn test_common_path_common_root() {
        let dir = temp_dir();
        let file1_path = dir.join("file1.txt");
        let file2_path = dir.join("file2.txt");
        fs::File::create(&file1_path).unwrap();
        fs::File::create(&file2_path).unwrap();

        let files = vec![file1_path.clone(), file2_path.clone()];
        let com_path = common_path(files);
        fs::remove_file(file1_path).unwrap();
        fs::remove_file(file2_path).unwrap();
        assert_eq!(com_path, Some(dir));
    }

    #[test]
    fn test_common_path_no_common_path() {
        let dir1 = temp_dir().join("folder1");
        let dir2 = temp_dir().join("folder2");
        fs::create_dir_all(&dir1).expect("Failed to create Dir");
        fs::create_dir_all(&dir2).expect("Failed to create Dir");
        let file1_path = dir1.join("file1.txt");
        let file2_path = dir2.join("file2.txt");
        fs::File::create(&file1_path).unwrap();
        fs::File::create(&file2_path).unwrap();

        let files = vec![file1_path.clone(), file2_path.clone()];
        let com_path = common_path(files);
        fs::remove_dir_all(dir1).unwrap();
        fs::remove_dir_all(dir2).unwrap();
        assert_eq!(com_path, Some(temp_dir()));
    }
}
