pub type UsageReferences = (UsageReferencesKey, Vec<String>);

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum UsageReferencesKey {
    #[serde(rename = "urefs")]
    Key,
}
