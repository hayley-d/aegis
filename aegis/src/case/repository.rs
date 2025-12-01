use crate::case::model::Case;
use tokio_postgres::Client;

pub struct CaseRepository<'a> {
    pub client: &'a Client,
}

impl<'a> CaseRepository<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
