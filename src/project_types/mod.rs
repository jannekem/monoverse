use serde::Deserialize;

pub mod node;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    Node,
}

pub trait ProjectFile {
    fn bump_version(&self) -> anyhow::Result<()>;
}
