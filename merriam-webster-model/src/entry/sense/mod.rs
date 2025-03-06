pub mod binding_substitute;
pub mod bold_italic_note;
pub mod defining_text;
pub mod definition_section;
pub mod divided_sense;
pub mod parenthesized_sense_sequence;
pub mod sense_number;
pub mod sense_sequence;
pub mod truncated_sense;
pub mod verb_divider;

use super::{
    boxed_supplemental_info_note::BoxedSupplementalInfoNote,
    etymology::Etymology,
    inflections::Inflections,
    labels::{GeneralLabels, SenseSpecificGrammaticalLabel, SubjectStatusLabels},
    phrasal_verb::PhrasalVerbs,
    pronunciations::Pronunciations,
    variants::Variants
};

use self::{defining_text::DefiningText, divided_sense::DividedSense, sense_number::SenseNumber, bold_italic_note::BoldItalicNote};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Senses {
    Sense(Box<Sense>),
    Senses(Vec<Senses>),
}

pub type Sense = (SenseKey, SenseObject);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SenseKey {
    #[serde(rename = "sense")]
    Key,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SenseObject {
    bnote: Option<BoldItalicNote>,
    dt: DefiningText,
    et: Option<Etymology>,
    ins: Option<Inflections>,
    lbs: Option<GeneralLabels>,
    phrasev: Option<PhrasalVerbs>,
    prs: Option<Pronunciations>,
    sdsense: Option<DividedSense>,
    sgram: Option<SenseSpecificGrammaticalLabel>,
    sls: Option<SubjectStatusLabels>,
    sn: Option<SenseNumber>,
    snotebox: Option<BoxedSupplementalInfoNote>,
    sphrasev: Option<PhrasalVerbs>,
    vrs: Option<Variants>,
}
