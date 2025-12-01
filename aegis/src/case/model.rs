use chrono::prelude::*;
use uuid::Uuid;
mod system;
use system::connection::connect;

#[derive(Debug, Clone)]
pub enum PhaseKind {
    Collection,
    Analysis,
    Reporting,
    Closure,
}

impl PhaseKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            PhaseKind::Collection => "collection",
            PhaseKind::Analysis => "analysis",
            PhaseKind::Reporting => "reporting",
            PhaseKind::Closure => "closure",
        }
    }

    pub fn from_str(string_form: &str) -> Self {
        match string_from {
            "collection" => Self::Collection,
            "analysis" => Self::Analysis,
            "reporting" => Self::Reporting,
            "closure" => Self::Colsure,
            _ => panic!("Invalid phase kind"),
        }
    }
}

pub struct Case {
    id: Uuid,
    name: String,
    created_at: DateTime<Utc>,
    created_by: Uuid,
    updated_at: DateTime<Utc>,
    category: String,
    description: String,
    phase: PhaseKind,
}

impl Case {
    pub fn new(
        name: &str,
        created_by: Uuid,
        category: &str,
        description: &str,
        phase: PhaseKind,
    ) -> Self {
        Case {
            id: Uuid::new_v4(),
            name: name.to_string(),
            created_at: Utc::now(),
            created_by: created_by.to_string(),
            updated_at: Utc::now(),
            category: category.to_string(),
            description: description.to_string(),
            phase,
        }
    }

    pub fn edit(
        id: Uuid,
        name: Option<String>,
        category: Option<PhaseKind>,
        description: Option<String>,
    ) -> Self {
    }
}
