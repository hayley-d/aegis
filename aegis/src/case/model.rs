use chrono::prelude::*;
use uuid::Uuid;

pub enum PhaseKind {
    Collection,
    Analysis,
    Reporting,
    Closure,
}

pub struct Case {
    id: UUID,
    name: String,
    created_at: DateTime,
    created_by: UUID,
    updated_at: DateTime,
    category: String,
    description: String,
    phase: PhaseKind,
}

impl case {
    fn new(
        name: &str,
        created_by: UUID,
        category: &str,
        description: &str,
        phase: PhaseKind,
    ) -> Self {
        Case {
            id: Uuid::new_v4(),
            name: name.clone(),
            created_at: Utc::now(),
            created_by: created_by.clone(),
            updated_at: Utc::now(),
            category: category.clone(),
            description: description.clone(),
            phase,
        }
    }
}
