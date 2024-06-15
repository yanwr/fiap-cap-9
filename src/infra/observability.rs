#[derive(Debug, Clone, Default, PartialEq)]
pub struct Tags {
    pub user_id: Option<String>,
}

pub trait ToTags {
    fn to_tags(&self) -> Tags;
}

impl ToTags for &Tags {
    fn to_tags(&self) -> Tags {
        Tags {
            user_id: self.user_id.clone(),
        }
    }
}
