use std::{fmt::Display, path::PathBuf};

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct Treemap<T> {
    pub node: T,
    pub depth: usize,
    pub branches: Vec<Box<Treemap<T>>>,
}

impl<T: Display> Display for Treemap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "-{} depth: {}", self.node, self.depth)?;
        for branch in &self.branches {
            write!(f, "| ")?;
            branch.fmt(f)?;
        }
        Ok(())
    }
}

impl Treemap<PathBuf> {
    pub fn display(&self, prev: &String) {
        let next = format!("{prev}|");
        print!("-{} depth: {}", self.node.display(), self.depth);
        print!("\n");
        for branch in &self.branches {
            print!("{}", &next);
            branch.display(&next);
        }
    }
}
