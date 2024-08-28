use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ChecklistItem {
    CheckClamscanSidecar,
    CheckFreshclamSidecar,
    CheckFreshclamConfig,
}

pub static CHECKLIST: LazyLock<Vec<ChecklistItem>> = LazyLock::new(|| {
    vec![
        ChecklistItem::CheckClamscanSidecar,
        ChecklistItem::CheckFreshclamSidecar,
        ChecklistItem::CheckFreshclamConfig,
    ]
});
