use object::Object;

pub struct Blob {
    content: Vec<u8>
}

impl Blob {
    pub fn new(conent: Vec<u8>) -> Blob {
        Blob { content: content }
    }

    pub fn length(&self) -> usize {
        self.content.len()
    }

    pub fn get_content(&self) -> Vec<u8> {
        self.content.clone()
    }

    pub fn get_metadata(&self) -> Vec<u8> {
        format!("blob {}", self.length()).into_bytes()
    }
}

impl Object for Blob {
    fn get_hash_content(&self) -> Vec<u8> {
        self.get_content()
    }
}

impl From<Vec<u8>> for Blob {
    fn from(data: Vec<u8>) -> Blob {
        let mut start = 4;
        for &byte in &data[4..] {
            start += 1;
            if byte == 0 {
                break;
            }
        }

        let mut vec = Vec::new();
        vec.extend_from_slice(&data[start..]);
        Blob::new(vec)
    }
}
