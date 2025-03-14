#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryMetadata {
    /// unique entry identifier within a particular dictionary data set, used in cross-references pointing to the entry.
    /// It consists of the headword, and in homograph entries, an appended colon and homograph number.
    pub id: String,
    /// universally unique identifier
    pub uuid: String,
    /// a 9-digit code which may be used to sort entries in the proper dictionary order if alphabetical display is needed
    pub sort: Option<String>,
    /// source data set for entry—ignore
    pub src: String,
    /// indicates the section the entry belongs to in print, where "alpha" indicates the main alphabetical section, "biog" biographical, "geog" geographical, and "fw&p" the foreign words & phrases section.
    pub section: String,
    /// lists all of the entry's headwords, variants, inflections, undefined entry words, and defined run-on phrases. Each stem string is a valid search term that should match this entry.
    pub stems: Vec<String>,
    /// true if there is a label containing "offensive" in the entry; otherwise, false.
    pub offensive: bool,
    // Below is to support Learner's Dictionary
    /// if text is "yes", the headword is a key part of English vocabulary that is highlighted in print
    #[serde(default)]
    pub highlight: Option<String>,
    /// a very abbreviated version of the entry that could be used in specialized contexts where a preview or shortened entry view is needed;
    #[serde(rename = "app-shortdef")]
    pub app_shortdef: Option<AppShortDef>,
    #[serde(default)]
    pub syns: Option<Vec<String>>,
    #[serde(default)]
    pub ants: Option<Vec<String>>,
    #[serde(default)]
    pub targets: Option<Target>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppShortDef {
    AppShortDefFull(AppShortDefFull),
    EmptyVec(EmptyVec),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppShortDefFull {
    pub hw: String,
    pub fl: String,
    pub def: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EmptyVec {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub tuuid: String,
    pub tsrc: String,
}
