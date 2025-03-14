pub mod binding_substitute;
pub mod bold_italic_note;
pub mod boxed_supplemental_info_notes;
pub mod defining_text;
pub mod definition_section;
pub mod divided_sense;
pub mod parenthesized_sense_sequence;
pub mod sense_number;
pub mod sense_sequence;
pub mod truncated_sense;
pub mod verb_divider;

use super::{
    etymology::Etymology,
    inflections::Inflections,
    labels::{GeneralLabels, SenseSpecificGrammaticalLabel, SubjectStatusLabels},
    phrasal_verb::PhrasalVerbs,
    pronunciations::Pronunciations,
    variants::Variants,
};

use self::{
    bold_italic_note::BoldItalicNote, boxed_supplemental_info_notes::BoxedSupplementalInfoNotes,
    defining_text::DefiningText, divided_sense::DividedSense, sense_number::SenseNumber,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Senses {
    Sense(Box<Sense>),
    Senses(Vec<Senses>),
}

pub type Sense = (SenseKey, SenseObject);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SenseKey {
    #[serde(rename = "sense")]
    Key,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SenseObject {
    pub bnote: Option<BoldItalicNote>,
    pub dt: DefiningText,
    pub et: Option<Etymology>,
    pub ins: Option<Inflections>,
    pub lbs: Option<GeneralLabels>,
    pub phrasev: Option<PhrasalVerbs>,
    pub prs: Option<Pronunciations>,
    pub sdsense: Option<DividedSense>,
    pub sgram: Option<SenseSpecificGrammaticalLabel>,
    pub sls: Option<SubjectStatusLabels>,
    pub sn: Option<SenseNumber>,
    pub snotebox: Option<BoxedSupplementalInfoNotes>,
    pub sphrasev: Option<PhrasalVerbs>,
    pub vrs: Option<Variants>,
}

impl Senses {
    pub fn get_inner(&self) -> Vec<&SenseObject> {
        let mut return_senses = vec![];
        match self {
            Self::Sense(s) => {
                let (key, object) = &*s.as_ref();
                return_senses.push(object);
            }
            _ => {
                self.get_inner();
            }
        }
        return_senses
    }
}
