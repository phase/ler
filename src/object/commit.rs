use object::Object;

pub struct Commit {
    pub tree: Rc<Tree>,
    pub parents: Vec<Rc<Commit>>,
    pub author: (Contact, TimeInfo),
    pub committer: (Contact, TimeInfo)
}

impl Object for Commit {

}