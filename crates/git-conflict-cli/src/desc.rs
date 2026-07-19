use ratatui::crossterm::style::{StyledContent, Stylize};

pub struct Description<'a> {
    index: String,
    content: Vec<StyledContent<&'a str>>,
}
impl<'a> Description<'a> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn local_desc(&self) -> Vec<StyledContent<&'a str>> {
        vec![
            "Accept Local changes and abondon foreign changes".white(),
            "The local changes are pointed by the Head or the branch that you are currently at".white(),
            "The changes of the other branch that you are targeting for a merge will be abondoned".white(),
            "The new merge commit will use the current branch's commit (The conflicted one) as the parent for the new conflict resolution commit".white(),
        ]
    }

    pub fn foreign_desc(&self) -> Vec<StyledContent<&'a str>> {
        vec![
            "Accept foreign changes and abondon local once".white(),
            "The foreign changes are pointed by the other branch that are trying to merge".white(),
            "The changes of the local branch that are pointed by the head (the branch you are currently at) will be abondoned".white(),
            "The new merge commit will use the current branch's commit (The conflicted one) as the parent for the new conflict resolution commit".white(),
        ]
    }

    pub fn combination_decs(&self) -> Vec<StyledContent<&'a str>> {
        vec![
            "Merge both Head and foreign branch's changes".white(),
            "The conflict is resolved through accepting both branches' changes".white(),
            "Both branches changes will be accepted and there won't be any abondoned changes".white(),
            "The new merge commit will contain both branches' commits as the parents for the new commit, the branch you are currently at will be the ancestor".white(),
        ]
    }

    pub fn tree_desc(&self) -> Vec<StyledContent<&'a str>> {
        vec![
            "Resolve conflict through tree index-based merging".white(),
            "If parent interference is enabled: The tree of the head branch or the foreign branch will be merged along side that parent tree".white(),
            "If parent interference is not enabled: The git index tree will be merged with either (Head, Foreign) Trees".white(),
            "Use this method when you want to have better control over your branches and avoid losing parent changes".white(),
            "The method is prefered to keep teams on track and avoid losing sync that is caused due to extremely divergent changes".white(),
        ]
    }
}
