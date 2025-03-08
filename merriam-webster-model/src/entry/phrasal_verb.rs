#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PhrasalVerbs {
    Sphrasev(SPhrasalVerb),
    Phrasev(PhrasalVerb),
}

pub type PhrasalVerb = Vec<InnerPhrasalVerb>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerPhrasalVerb {
    /// phrasal verb (required)
    #[serde(rename = "pva")]
    pub phrasal_verb: String,
    #[serde(rename = "pvl", default)]
    pub phrasal_verb_label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPhrasalVerb {
    pub phrs: Vec<InnerPhrasalVerb>,
    #[serde(rename = "phsls", default)]
    pub phrasal_verb_subject_label: Option<Vec<String>>,
}
