use crate::entry::verbal_illustrations::VerbalIllustrations;

pub type BoxedSupplementalInfoNotes = Vec<BoxedSupplementalInfoNoteType>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde[untagged]]
pub enum BoxedSupplementalInfoNoteType {
    Text(BoxedSupplementalInfoNoteText),
    VerbalIllustrations(VerbalIllustrations),
}

pub type BoxedSupplementalInfoNoteText = (BoxedSupplementalInfoNoteKey, String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BoxedSupplementalInfoNoteKey {
    #[serde(rename = "t")]
    Key,
}

