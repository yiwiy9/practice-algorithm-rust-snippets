use cargo_snippet::snippet;

#[allow(dead_code)]
#[snippet("trie")]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_end: bool,
}

#[allow(dead_code)]
#[snippet("trie")]
impl Trie {
    fn new() -> Self {
        Self {
            children: Default::default(),
            is_end: false,
        }
    }

    fn insert(&mut self, s: &str) {
        let mut node = self;
        for b in s.bytes() {
            let i = (b - b'a') as usize;
            node = node.children[i].get_or_insert_with(|| Box::new(Trie::new()));
        }
        node.is_end = true;
    }

    fn contains(&self, s: &str) -> bool {
        let mut node = self;
        for b in s.bytes() {
            let i = (b - b'a') as usize;
            match &node.children[i] {
                Some(next) => node = next,
                None => return false,
            }
        }
        node.is_end
    }

    fn starts_with(&self, prefix: &str) -> bool {
        let mut node = self;
        for b in prefix.bytes() {
            let i = (b - b'a') as usize;
            match &node.children[i] {
                Some(next) => node = next,
                None => return false,
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie_insert_and_contains() {
        let mut trie = Trie::new();
        trie.insert("apple");
        trie.insert("app");
        trie.insert("banana");

        assert!(trie.contains("apple"));
        assert!(trie.contains("app"));
        assert!(trie.contains("banana"));
        assert!(!trie.contains("banan"));
        assert!(!trie.contains("apples"));
        assert!(!trie.contains("orange"));
    }

    #[test]
    fn test_trie_starts_with() {
        let mut trie = Trie::new();
        trie.insert("apple");
        trie.insert("app");
        trie.insert("banana");

        assert!(trie.starts_with("app"));
        assert!(trie.starts_with("ban"));
        assert!(trie.starts_with("bana"));
        assert!(!trie.starts_with("bat"));
        assert!(!trie.starts_with("cat"));
    }
}
