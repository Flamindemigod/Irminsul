use crate::utils::treemap::Treemap;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, time::Duration};

use super::Poll;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Mix {
    pub poll_rate: Duration,
    pub branch_depth_ratio: f32,
}

impl Default for Mix {
    fn default() -> Self {
        Self {
            poll_rate: Duration::from_millis(200),
            branch_depth_ratio: 4.0,
        }
    }
}

impl Poll for Mix {
    fn poll(&self, path_map: &mut Vec<Box<Treemap>>) -> Option<Vec<PathBuf>> {
        let res: Vec<PathBuf> = path_map
            .par_iter_mut()
            .map(|p| {
                p.poll_map(self.branch_depth_ratio)
                    .par_iter_mut()
                    .map(|point| point.poll_branches())
                    .flatten()
                    .collect::<Vec<PathBuf>>()
            })
            .flatten()
            .collect();
        if !res.is_empty() {
            return Some(res);
        } else {
            return None;
        }
    }
}

impl Treemap {
    fn poll_map(&mut self, branch_depth_ratio: f32) -> Vec<&mut Self> {
        if self.branches.is_empty() {
            return vec![self];
        } else {
            if (self.branches.len() as f32 / (self.depth + 1) as f32) > branch_depth_ratio {
                return vec![self];
            } else {
                return self
                    .branches
                    .par_iter_mut()
                    .map(|t| t.poll_map(branch_depth_ratio))
                    .flatten()
                    .collect::<Vec<_>>();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::path_map;

    use super::*;
    use std::{env::temp_dir, fs, io::Write, thread::sleep, time::Duration};

    #[test]
    fn test_poll_empty_input() {
        let mut inp = Vec::new();
        assert_eq!(Mix::default().poll(&mut inp), None);
    }

    #[test]
    fn test_poll_1file_1pass() {
        let dir = temp_dir();
        let file_path1 = dir.join("file_mix1_poll1.txt");
        fs::File::create(&file_path1).unwrap();
        let files = vec![file_path1.clone()];
        let mut inp = path_map(files);
        let mut l = fs::File::create(&file_path1).unwrap();
        let _ = writeln!(l, "Test");
        drop(l);
        assert_eq!(
            Mix::default().poll(&mut inp),
            Some(vec![file_path1.clone()])
        );
        fs::remove_file(file_path1.clone()).unwrap();
    }

    #[test]
    fn test_poll_2file_1pass() {
        let dir = temp_dir();
        let file_path1 = dir.join("file_mix2_poll1.txt");
        let file_path2 = dir.join("file_mix2_poll2.txt");
        fs::File::create(&file_path1).unwrap();
        fs::File::create(&file_path2).unwrap();
        let files = vec![file_path1.clone(), file_path2.clone()];
        let mut inp = Box::new(path_map(files));
        sleep(Duration::from_millis(500));
        let mut l = fs::File::create(&file_path1).unwrap();
        let _ = writeln!(l, "Test");
        drop(l);
        assert_eq!(
            Mix::default().poll(&mut inp),
            Some(vec![file_path1.clone()])
        );
        fs::remove_file(file_path1.clone()).unwrap();
        fs::remove_file(file_path2.clone()).unwrap();
    }

    #[test]
    fn test_poll_2file_2pass() {
        let dir = temp_dir();
        let file_path1 = dir.join("file_mix3_poll1.txt");
        let file_path2 = dir.join("file_mix3_poll2.txt");
        fs::File::create(&file_path1).unwrap();
        fs::File::create(&file_path2).unwrap();
        let files = vec![file_path1.clone(), file_path2.clone()];
        let mut inp = path_map(files);
        sleep(Duration::from_millis(500));
        let mut l = fs::File::create(&file_path1).unwrap();
        let _ = writeln!(l, "Test");
        drop(l);
        let mut l = fs::File::create(&file_path2).unwrap();
        let _ = writeln!(l, "Test");
        drop(l);
        assert_unordered::assert_eq_unordered!(
            Mix::default().poll(&mut inp).expect("Failed Test"),
            vec![file_path1.clone(), file_path2.clone()]
        );
        fs::remove_file(file_path1.clone()).unwrap();
        fs::remove_file(file_path2.clone()).unwrap();
    }

    #[test]
    fn test_poll_2file_0pass() {
        let dir = temp_dir();
        let file_path1 = dir.join("file_mix4_poll1.txt");
        let file_path2 = dir.join("file_mix4_poll2.txt");
        fs::File::create(&file_path1).unwrap();
        fs::File::create(&file_path2).unwrap();
        let files = vec![file_path1.clone(), file_path2.clone()];
        let mut inp = path_map(files);
        sleep(Duration::from_millis(500));
        assert_eq!(Mix::default().poll(&mut inp), None);
        fs::remove_file(file_path1.clone()).unwrap();
        fs::remove_file(file_path2.clone()).unwrap();
    }
}
