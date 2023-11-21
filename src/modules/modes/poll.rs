use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, time::Duration};

use crate::modules::treemap::Treemap;

use super::{Poll as PollTrait, PollMap};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Poll {
    poll_rate: Duration,
}

impl Default for Poll {
    fn default() -> Self {
        Self {
            poll_rate: Duration::from_millis(500),
        }
    }
}

impl PollTrait for Poll {
    fn poll(&self, path_map: &mut Box<Treemap>) -> Option<Vec<std::path::PathBuf>> {
        let res = <Treemap as PollMap<Poll>>::poll_map(path_map, 0.0, 0)
            .par_iter_mut()
            .map(|point| point.poll_branches())
            .flatten()
            .collect::<Vec<PathBuf>>();
        if !res.is_empty() {
            return Some(res);
        } else {
            return None;
        }
    }
}

impl PollMap<Poll> for Treemap {
    fn poll_map(&mut self, _: f32, _: usize) -> Vec<&mut Self> {
        if self.branches.is_empty() {
            return vec![self];
        } else {
            return self
                .branches
                .par_iter_mut()
                .map(|t| <Treemap as PollMap<Poll>>::poll_map(t, 0.0, 0))
                .flatten()
                .collect::<Vec<_>>();
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use serial_test::serial;
    use std::{env::temp_dir, fs, io::Write, thread::sleep, time::Duration};

    use super::*;
    use crate::modules::node::Node;

    #[test]
    #[serial]
    fn test_poll_poll_no_update() {
        let dir = temp_dir().join("poll_poll_test0");
        let _ = fs::create_dir_all(dir.clone());
        let mut map = Node {
            root: dir.clone(),
            path_pattern: None,
            exec: None,
        }
        .build_treemap();
        assert_eq!(Poll::default().poll(&mut map), None);
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn test_poll_poll_1file_1pass() {
        let dir = temp_dir().join("poll_poll_test1");
        let _ = fs::create_dir_all(dir.clone());
        let file_path1 = dir.join("file_poll1_poll1.txt");
        fs::File::create(&file_path1).unwrap();
        let mut map = Node {
            root: dir.clone(),
            path_pattern: Some("*".to_owned()),
            exec: None,
        }
        .build_treemap();
        sleep(Duration::from_millis(500));

        let mut l = fs::File::create(&file_path1).unwrap();
        let _ = writeln!(l, "Test");
        drop(l);
        assert_eq!(Poll::default().poll(&mut map), Some(vec![file_path1]));
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    #[serial]
    fn test_poll_poll_2file_1pass() {
        let dir = temp_dir().join("poll_poll_test2");
        let _ = fs::create_dir_all(dir.clone());
        let file_path1 = dir.join("file_poll2_poll1.txt");
        let file_path2 = dir.join("file_poll2_poll2.txt");
        fs::File::create(&file_path1).unwrap();
        fs::File::create(&file_path2).unwrap();
        let mut map = Node {
            root: dir.clone(),
            path_pattern: Some("*".to_owned()),
            exec: None,
        }
        .build_treemap();
        sleep(Duration::from_millis(500));
        let mut l = fs::File::create(&file_path1).unwrap();
        let _ = writeln!(l, "Test");
        drop(l);
        assert_eq!(
            Poll::default().poll(&mut map),
            Some(vec![file_path1.clone()])
        );
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    #[serial]
    fn test_poll_poll_2file_2pass() {
        let dir = temp_dir().join("poll_poll_test3");
        let _ = fs::create_dir_all(dir.clone());
        let file_path1 = dir.join("file_poll3_poll1.txt");
        let file_path2 = dir.join("file_poll3_poll2.txt");
        fs::File::create(&file_path1).unwrap();
        fs::File::create(&file_path2).unwrap();
        let mut map = Node {
            root: dir.clone(),
            path_pattern: Some("*".to_owned()),
            exec: None,
        }
        .build_treemap();
        sleep(Duration::from_millis(500));
        let mut l = fs::File::create(&file_path1).unwrap();
        let _ = writeln!(l, "Test");
        drop(l);

        let mut l = fs::File::create(&file_path2).unwrap();
        let _ = writeln!(l, "Test");
        drop(l);

        let mut res = Poll::default().poll(&mut map).unwrap();
        let mut exp = vec![file_path1.clone(), file_path2.clone()];
        exp.sort();
        res.sort();
        assert_eq!(res, exp);
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    #[serial]
    fn test_poll_poll_2file_0pass() {
        let dir = temp_dir().join("poll_poll_test4");
        let _ = fs::create_dir_all(dir.clone());
        let file_path1 = dir.join("file_poll4_poll1.txt");
        let file_path2 = dir.join("file_poll4_poll2.txt");
        fs::File::create(&file_path1).unwrap();
        fs::File::create(&file_path2).unwrap();
        let mut map = Node {
            root: dir.clone(),
            path_pattern: Some("*".to_owned()),
            exec: None,
        }
        .build_treemap();

        assert_eq!(Poll::default().poll(&mut map), None);
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    #[serial]
    fn test_poll_poll_3file_2pass() {
        let dir = temp_dir().join("poll_poll_test5");
        let _ = fs::create_dir_all(dir.clone().join("inner"));
        let file_path1 = dir.join("file_poll5_poll1.txt");
        let file_path2 = dir.join("file_poll5_poll2.txt");
        let file_path3 = dir.join("inner").join("file_poll5_poll2.txt");
        fs::File::create(&file_path1).unwrap();
        fs::File::create(&file_path2).unwrap();
        fs::File::create(&file_path3).unwrap();
        let mut map = Node {
            root: dir.clone(),
            path_pattern: Some("**/*".to_owned()),
            exec: None,
        }
        .build_treemap();
        sleep(Duration::from_millis(500));
        let mut l = fs::File::create(&file_path1).unwrap();
        let _ = writeln!(l, "Test");
        drop(l);

        let mut l = fs::File::create(&file_path2).unwrap();
        let _ = writeln!(l, "Test");
        drop(l);

        let mut res = Poll::default().poll(&mut map).unwrap();
        let mut exp = vec![file_path1.clone(), file_path2.clone()];
        exp.sort();
        res.sort();
        assert_eq!(res, exp);
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    #[serial]
    fn test_poll_poll_3file_1pass_inner() {
        let dir = temp_dir().join("poll_poll_test6");
        let _ = fs::create_dir_all(dir.clone().join("inner"));
        let file_path1 = dir.join("file_poll6_poll1.txt");
        let file_path2 = dir.join("file_poll6_poll2.txt");
        let file_path3 = dir.join("inner").join("file_poll6_poll2.txt");
        fs::File::create(&file_path1).unwrap();
        fs::File::create(&file_path2).unwrap();
        fs::File::create(&file_path3).unwrap();
        let mut map = Node {
            root: dir.clone(),
            path_pattern: Some("**/*".to_owned()),
            exec: None,
        }
        .build_treemap();
        sleep(Duration::from_millis(500));
        let mut l = fs::File::create(&file_path3).unwrap();
        let _ = writeln!(l, "Test");
        drop(l);

        let mut res = Poll::default().poll(&mut map).unwrap();
        let mut exp = vec![file_path3];
        exp.sort();
        res.sort();
        assert_eq!(res, exp);
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    #[serial]
    fn test_poll_poll_map() {
        let dir = temp_dir().join("poll_poll_test6");
        let _ = fs::create_dir_all(dir.clone().join("inner"));
        let file_path1 = dir.join("file_poll6_poll1.txt");
        let file_path2 = dir.join("file_poll6_poll2.txt");
        let file_path3 = dir.join("inner").join("file_poll6_poll2.txt");
        fs::File::create(&file_path1).unwrap();
        fs::File::create(&file_path2).unwrap();
        fs::File::create(&file_path3).unwrap();
        let mut map = Node {
            root: dir.clone(),
            path_pattern: Some("**/*".to_owned()),
            exec: None,
        }
        .build_treemap();

        {
            let res = <Treemap as PollMap<Poll>>::poll_map(&mut map, 2.0, 0);
            assert_eq!(res.len(), 3);
        }

        let _ = fs::remove_dir_all(dir);
    }
}
