pub type UsageReferences = (UsageReferencesKey, Vec<String>);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UsageReferencesKey {
    #[serde(rename = "urefs")]
    Key,
}