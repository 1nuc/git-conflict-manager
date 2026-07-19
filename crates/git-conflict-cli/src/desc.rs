use ratatui::{style::Style, text::{Line, Span}};

#[derive(Default)]
pub struct Description<'a> {
    pub content: Line<'a>,
}

impl<'a> Description<'a> {
    pub fn init(&self, index: String) -> Self {
        let content= match index.as_str(){
            "0" => {
                self.local_desc()
            },
            "1" => {
                self.foreign_desc()
            },
            "2" => {
                self.combination_decs()
            },
            "3" => {
                self.tree_desc()
            },
            _ => Line::default()
        };
        Self{
            content,
        }
    }

    pub fn local_desc(&self) -> Line<'a>{
        Line::from(vec![
            Span::styled("Accept Local changes and abondon foreign changes.", Style::new().white()),
            Span::styled("The local changes are pointed by the Head or the branch that you are currently at.", Style::new().white()),
            Span::styled("The changes of the other branch that you are targeting for a merge will be abondoned.", Style::new().white()),
            Span::styled("The new merge commit will use the current branch's commit (The conflicted one)", Style::new().white()),
            Span::styled("as the parent for the new conflict resolution commit.", Style::new().white()),
            Span::styled("This method is prefered when you want to keep your local branch's changes ignoring the other branch.", Style::new().white()),
        ])
    }

    pub fn foreign_desc(&self) -> Line<'a>{
        Line::from(vec![
            Span::styled("Accept foreign changes and abondon local once.", Style::new().white()),
            Span::styled("The foreign changes are pointed by the other branch that you are trying to merge.", Style::new().white()),
            Span::styled("The changes of the local branch that are pointed by the head", Style::new().white()),
            Span::styled("(the branch you are currently at) will be abondoned.", Style::new().white()),
            Span::styled("The new merge commit will use the current branch's commit", Style::new().white()),
            Span::styled("as the parent for the new conflict resolution commit.", Style::new().white()),
            Span::styled("This method is prefered when you want to ignore your local branch's changes and accept the foreign updates.", Style::new().white()),
        ])
    }

    pub fn combination_decs(&self) ->Line<'a> {
        Line::from(vec![
            Span::styled("Merge both Head and foreign branch's changes.", Style::new().white()),
            Span::styled("The conflict is resolved through accepting both branches' changes.", Style::new().white()),
            Span::styled("Both branches changes will be accepted and there won't be any abondoned changes.", Style::new().white()),
            Span::styled("The new merge commit will contain both branches' commits as the parents for the new commit.", Style::new().white()),
            Span::styled("The branch you are currently at will be the ancestor", Style::new().white()),
            Span::styled("This method is prefered when you want to merge both branch's updates.", Style::new().white()),
        ])
    }

    pub fn tree_desc(&self) -> Line<'a>{
        Line::from(vec![
            Span::styled("Resolve conflict through tree index-based merging.", Style::new().white()),
            Span::styled("If parent interference is enabled: The tree of the head branch", Style::new().white()),
            Span::styled("or the foreign branch will be merged along side that parent tree.", Style::new().white()),
            Span::styled("If parent interference is not enabled: ", Style::new().white()),
            Span::styled("The git index tree will be merged with either (Head, Foreign) Trees.", Style::new().white()),
            Span::styled("Use this method when you want to have better control over your branches,", Style::new().white()),
            Span::styled("and avoid losing parent changes.", Style::new().white()),
            Span::styled("The method is prefered to keep teams on track, ", Style::new().white()),
            Span::styled("and avoid losing sync that is caused due to extremely divergent changes.", Style::new().white()),
        ])
    }
}
