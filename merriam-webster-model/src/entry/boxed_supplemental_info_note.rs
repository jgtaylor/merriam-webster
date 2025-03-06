use super::verbal_illustrations::VerbalIllustrations;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoxedSupplementalInfoNote {
    #[serde(rename = "t")]
    pub text: String,
    pub verbal_illustrations: Option<VerbalIllustrations>
}