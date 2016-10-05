use object::Object;

pub struct Tree<'a> {
    entries: Vec<&'a TreeNode<'a>>
}

impl<'a> Tree<'a> {
    pub fn new() -> Tree<'a> {
        Tree { entries: Vec::new() }
    }

    pub fn add(&mut self, obj: &'a TreeNode) -> &Tree {
        self.entries.push(obj);
        self
    }
}

impl<'a> Object for Tree<'a> {
    fn get_hash_content(&self) -> Vec<u8> {
        let buffer = entries.iter()
            .fold(Vec::new(), |mut buffer, entry| {
                buffer.extend_from_slice(format!("{:o} {}", entry.mode, entry.name).as_bytes());
                buffer.push(0);
                buffer.extend(entry.object.hash());
                buffer
            });
        let mut result = format!("blob {}", buffer.len()).into_bytes();
        result.push(0);
        result.extend(buffer);
        result
    }
}

pub struct TreeNode<'a> {
    pub object: &'a Object,
    pub name: String
}

impl<'a> TreeNode<'a> {
    pub fn new(object: &'a Object, name: String) -> TreeNode<'a> {
        TreeNode { object: object, name: name }
    }
}
