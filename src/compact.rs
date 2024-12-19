use std::convert::TryInto;
use serde::{Deserialize, Serialize};
use crate::{matching, Error, Zipcode};

#[derive(Debug, Serialize, Deserialize)]
pub struct ZipCodeCompact<'a>(&'a str);

impl<'a> ZipCodeCompact<'a> {
    pub fn expand(&self) -> crate::Result<Option<Zipcode>> {
        matching(self.0, None).map(|f| f.get(0).cloned())
    }
}

impl TryInto<Zipcode> for ZipCodeCompact<'_> {
    type Error = Error;
    fn try_into(self) -> Result<Zipcode, Self::Error> {
        self.expand().and_then(|f| f.ok_or(Error::NotInDatabase))
    }
}