use crate::ComponentView;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeGenerationRequest {
    pub execution_id: String,
    pub handler: String,
    pub component: ComponentView,
    pub code_base64: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CodeGenerated {
    pub format: String,
    pub code: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeGenerationResultSuccess {
    pub execution_id: String,
    pub data: CodeGenerated,
    #[serde(default = "crate::timestamp")]
    pub timestamp: u64,
}
