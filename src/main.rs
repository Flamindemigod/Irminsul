use modules::modes::{mix::Mix, Poll};
use std::{path::PathBuf, thread::sleep, time::Duration};
use utils::path_map;

pub mod modules;
pub mod utils;

fn main() {
    let mut map = path_map(
        vec![
            r"E:\Projects\watch-rs\src\modules\config.rs",
            r"E:\Projects\watch-rs\src\modules\modes\poll.rs",
            r".\src\utils\treemap.rs",
            r".\src\utils\mod.rs",
            r".\src\utils\pathmap.rs",
            r".\src\main.rs",
            r".\src\modules\mod.rs",
            r".\Readme.md",
            r"test",
            r".\src\modules\modes\mix.rs",
            r"E:\Steam\steamapps\common\Blender",
        ]
        .iter()
        .map(|f| PathBuf::from(f))
        .collect(),
    );
    loop {
        sleep(Duration::from_millis(200));
        let res = Mix::default().poll(&mut map);
        match res {
            None => (),
            Some(paths) => println!("paths updated {:#?}", paths),
        }
    }
    // for root in map {
    //     root.display(&"".to_owned());
    // }
    // println!("{:#?}", map)
}
