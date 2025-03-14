use super::sense_number::SenseNumber;
use crate::entry::{
    etymology::Etymology,
    inflections::Inflections,
    labels::{GeneralLabels, SenseSpecificGrammaticalLabel, SubjectStatusLabels},
    pronunciations::Pronunciations,
    variants::Variants,
};

pub type TruncatedSense = (TruncatedSenseKey, TruncatedSenseObject);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TruncatedSenseObject {
    pub et: Option<Etymology>,
    pub ins: Option<Inflections>,
    pub lbs: Option<GeneralLabels>,
    pub prs: Option<Pronunciations>,
    pub sgram: Option<SenseSpecificGrammaticalLabel>,
    pub sls: Option<SubjectStatusLabels>,
    pub sn: Option<SenseNumber>,
    pub vrs: Option<Variants>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TruncatedSenseKey {
    #[serde(rename = "sen")]
    Key,
}
