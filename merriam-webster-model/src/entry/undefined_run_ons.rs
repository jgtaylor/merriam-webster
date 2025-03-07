use super::{
    inflections::Inflections, labels::{
        FunctionalLabel, GeneralLabels, ParenthesizedSubjectStatusLabel, SubjectStatusLabels,
    }, pronunciations::{AlternatePronounciation, Pronunciations}, sense::defining_text::WithinSenseGram, usage_notes::UsageNotes, variants::Variants, verbal_illustrations::VerbalIllustrations
};

pub type UndefinedRunOns = Vec<UndefinedRunOn>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndefinedRunOn {
    #[serde(rename = "ure")]
    pub name: Option<String>,
    #[serde(rename = "fl")]
    pub functional_label: FunctionalLabel,
    #[serde(rename = "utxt")]
    pub text: Option<Vec<UndefinedRunOnText>>,
    #[serde(rename = "ins")]
    pub inflections: Option<Inflections>,
    #[serde(rename = "lbs")]
    pub labels: Option<GeneralLabels>,
    #[serde(rename = "prs")]
    pub pronunciations: Option<Pronunciations>,
    #[serde(rename = "psl")]
    pub parenthesized_subect_status_label: Option<ParenthesizedSubjectStatusLabel>,
    #[serde(rename = "sls")]
    pub subject_status_labels: Option<SubjectStatusLabels>,
    #[serde(rename = "rsl")]
    pub run_on_subject_status_label: Option<String>,
    #[serde(rename = "vrs")]
    pub variants: Option<Variants>,
    #[serde(rename = "altprs", default)]
    pub alternate_pronounciation: Option<AlternatePronounciation>,
    #[serde(default)]
    pub gram: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UndefinedRunOnText {
    VerbalIllustrations(VerbalIllustrations),
    UsageNotes(UsageNotes),
    Wsgram(WithinSenseGram),
}
