use super::SenseObject;

pub type BindingSubstitute = (BindingSubstituteKey, InnerBindingSubstitute);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerBindingSubstitute {
    pub sense: SenseObject,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BindingSubstituteKey {
    #[serde(rename = "bs")]
    Key,
}

pub trait GetInner {
    fn get_inner(&self) -> Vec<&SenseObject>;
}

impl GetInner for BindingSubstitute {
    fn get_inner(&self) -> Vec<&SenseObject> {
        let mut return_senses = vec![];
        let (key, object) = &*self;
        return_senses.push(&object.sense);
        return_senses
    }
}
