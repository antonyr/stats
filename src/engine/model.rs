use serde_derive;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Root {
    sha: String,
    node_id: String,
    pub commit: Commit,
    url: String,
    html_url: String,
    comments_url: String,
    author: Author2,
    committer: Option<Committer2>,
    parents: Vec<Parent>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Commit {
    pub author: Author,
    committer: Committer,
    message: String,
    tree: Tree,
    url: String,
    comment_count: i64,
    verification: Verification,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Author {
    name: String,
    email: String,
    pub date: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Committer {
    name: String,
    email: String,
    date: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Tree {
    sha: String,
    url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Verification {
    verified: bool,
    reason: String,
    signature: Option<String>,
    payload: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Author2 {
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    type_field: String,
    site_admin: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Committer2 {
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    type_field: String,
    site_admin: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Parent {
    sha: String,
    url: String,
    html_url: String,
}
