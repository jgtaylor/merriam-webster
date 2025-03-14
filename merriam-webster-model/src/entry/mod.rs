pub mod sense;

pub mod alternate_headwords;
pub mod artwork;
pub mod attribution_of_quote;
pub mod biographical_name_wrap;
pub mod called_also_note;
pub mod cognate_cross_references;
pub mod defined_run_ons;
pub mod directional_cross_reference_section;
pub mod etymology;
pub mod first_known_use;
pub mod headword_information;
pub mod homograph;
pub mod inflections;
pub mod labels;
pub mod meta;
pub mod phrasal_verb;
pub mod pronunciations;
pub mod quotation_section;
pub mod run_in;
pub mod short_definitions;
pub mod supplemental_information_note;
pub mod synonym_section;
pub mod tables;
pub mod undefined_run_ons;
pub mod usage_notes;
pub mod usage_references;
pub mod usage_section;
pub mod variants;
pub mod verbal_illustrations;

pub use sense::{Sense, SenseObject};

use alternate_headwords::AlternateHeadwords;
use artwork::{Artwork, ArtworkLearners};
use cognate_cross_references::CognateCrossReferences;
use defined_run_ons::DefinedRunOns;
use directional_cross_reference_section::DirectionalCrossReferenceSection;
use etymology::Etymology;
use first_known_use::FirstKnownUse;
use headword_information::HeadwordInformation;
use homograph::Homograph;
use inflections::Inflections;
use labels::{FunctionalLabel, GeneralLabels};
use meta::EntryMetadata;
use quotation_section::QuotationSection;
use sense::{
    binding_substitute::{BindingSubstitute, GetInner},
    defining_text::DefiningText,
    definition_section::DefinitionSections,
    sense_sequence::{SenseSequenceType, SenseSequences},
    truncated_sense::TruncatedSenseObject,
};
use short_definitions::ShortDefinitions;
use synonym_section::SynonymSection;
use tables::Table;
use undefined_run_ons::UndefinedRunOns;
use usage_section::UsageSection;
use variants::Variants;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub meta: EntryMetadata,
    pub hwi: HeadwordInformation,
    #[serde(default)]
    pub hom: Option<Homograph>,
    #[serde(default)]
    pub ahws: Option<AlternateHeadwords>,
    #[serde(default)]
    pub vrs: Option<Variants>,
    #[serde(default)]
    pub fl: Option<FunctionalLabel>,
    #[serde(default)]
    pub lbs: Option<GeneralLabels>,
    #[serde(default)]
    pub ins: Option<Inflections>,
    #[serde(default)]
    pub cxs: Option<CognateCrossReferences>,
    #[serde(default)]
    pub def: Option<DefinitionSections>,
    #[serde(default)]
    pub uros: Option<UndefinedRunOns>,
    #[serde(default)]
    pub dros: Option<DefinedRunOns>,
    #[serde(default)]
    pub et: Option<Etymology>,
    #[serde(default)]
    pub dxnls: Option<DirectionalCrossReferenceSection>,
    #[serde(default)]
    pub usages: Option<UsageSection>,
    #[serde(default)]
    pub syns: Option<SynonymSection>,
    #[serde(default)]
    pub quotes: Option<QuotationSection>,
    #[serde(default)]
    pub art: Option<Artwork>,
    #[serde(default)]
    pub table: Option<Table>,
    #[serde(default)]
    pub date: Option<FirstKnownUse>,
    #[serde(default)]
    pub shortdef: Option<ShortDefinitions>,
    #[serde(default)]
    pub gram: Option<String>,
    #[serde(default)]
    pub artl: Option<ArtworkLearners>,
}

impl Entry {
    pub fn get_senses(&self) -> Vec<&SenseObject> {
        if let Some(ref def) = self.def {
            let mut return_sense_objects = vec![];
            for (idx, senses) in def.iter().enumerate() {
                match &senses.sense_sequence {
                    SenseSequences::SenseSequence(s) => {
                        for seq in s.iter() {
                            match seq {
                                SenseSequenceType::Senses(s) => {
                                    for i in s.get_inner() {
                                        return_sense_objects.push(i);
                                    }
                                }
                                SenseSequenceType::BindingSubstitute(s) => {
                                    for i in s.get_inner() {
                                        return_sense_objects.push(&i);
                                    }
                                }
                                SenseSequenceType::TruncatedSenseObject(s) => {
                                    let x = &s.clone();
                                    return_sense_objects.push(&SenseObject::from(x));
                                }
                                SenseSequenceType::TruncatedSense(s) => {
                                    let object = &s.1;
                                    return_sense_objects.push(&SenseObject::from(object));
                                }
                                SenseSequenceType::ParenthesizedSenseSequence(s) => {
                                    let object = &s.1;
                                    for i in object {
                                        match i {
                                            sense::parenthesized_sense_sequence::InnerParenthesizedSenseSequence::BindingSubstitute(s) => {
                                                for i in s.get_inner() {
                                                    return_sense_objects.push(i);
                                                }
                                            },
                                            sense::parenthesized_sense_sequence::InnerParenthesizedSenseSequence::BindingSubstitutes(s) => {
                                                for object in s {
                                                    for i in object.get_inner() {
                                                        return_sense_objects.push(i);
                                                    }
                                                }
                                            },
                                            sense::parenthesized_sense_sequence::InnerParenthesizedSenseSequence::Sense(s) => {
                                                let x = &s.1;
                                                return_sense_objects.push(x);
                                            },
                                            sense::parenthesized_sense_sequence::InnerParenthesizedSenseSequence::Senses(s) => {
                                                for i in s {
                                                    for x in i.get_inner() {
                                                        return_sense_objects.push(x);
                                                    }
                                                }

                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    SenseSequences::SenseSequences(s) => {}
                }
            }
            return_sense_objects // I DON'T FUKCING UNDERSTAND WHAT'S WRONG HERE!, i mean, i sort of do... but... ugh.
        } else {
            return vec![];
        }
    }
}

impl From<TruncatedSenseObject> for SenseObject {
    fn from(tso: TruncatedSenseObject) -> Self {
        SenseObject {
            bnote: None,
            dt: DefiningText::new(),
            et: tso.et,
            ins: tso.ins,
            lbs: tso.lbs,
            phrasev: None,
            prs: tso.prs,
            sdsense: None,
            sgram: tso.sgram,
            sls: tso.sls,
            sn: tso.sn,
            snotebox: None,
            sphrasev: None,
            vrs: tso.vrs,
        }
    }
}
impl From<&TruncatedSenseObject> for SenseObject {
    fn from(tso: &TruncatedSenseObject) -> Self {
        SenseObject {
            bnote: None,
            dt: DefiningText::new(),
            et: tso.et.to_owned(),
            ins: tso.ins.to_owned(),
            lbs: tso.lbs.to_owned(),
            phrasev: None,
            prs: tso.prs.to_owned(),
            sdsense: None,
            sgram: tso.sgram.to_owned(),
            sls: tso.sls.to_owned(),
            sn: tso.sn.to_owned(),
            snotebox: None,
            sphrasev: None,
            vrs: tso.vrs.to_owned(),
        }
    }
}
