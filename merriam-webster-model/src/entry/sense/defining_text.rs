use crate::entry::{
    biographical_name_wrap::BiographicalNameWrap, called_also_note::CalledAlsoNote, run_in::RunIn,
    supplemental_information_note::SupplementalInformationNote, usage_notes::UsageNotes,
    usage_references::UsageReferences, verbal_illustrations::VerbalIllustrations,
};

pub type DefiningText = Vec<DefiningTextType>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DefiningTextType {
    DefiningTextObject(DefiningTextObject),
    BiographicalNameWrap(BiographicalNameWrap),
    CalledAlsoNote(CalledAlsoNote),
    RunIn(RunIn),
    SupplementalInformationNote(SupplementalInformationNote),
    UsageNotes(UsageNotes),
    VerbalIllustrations(VerbalIllustrations),
    WithinSenseGram(WithinSenseGram),
    UsageReferences(UsageReferences),
}

pub type DefiningTextObject = (DefiningTextObjectKey, String);

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DefiningTextObjectKey {
    #[serde(rename = "text")]
    Key,
}

pub type WithinSenseGram = (WithinSenseGramObjectKey, String);

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum WithinSenseGramObjectKey {
    #[serde(rename = "wsgram")]
    Key,
}
