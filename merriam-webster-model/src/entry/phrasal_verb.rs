
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PhrasalVerbs {
    Sphrasev(SPhrasalVerb),
    Phrasev(PhrasalVerb)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerPhrasalVerb{
    /// phrasal verb (required)
    #[serde(rename = "pva")]
    pub phrasal_verb: String,
    #[serde(rename = "pvl")]
    pub phrasal_verb_label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "sphrasev")]
pub struct SPhrasalVerb{
    pub phrs: Vec<InnerPhrasalVerb>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "phrasev")]
pub struct PhrasalVerb {
    pub phr: InnerPhrasalVerb,
}


