use serde::{Deserialize, Serialize};

use crate::models::shared::permission_name::PermissionName;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionItems {
    pub name: PermissionName,
}
