use crate::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub mod cql {
    use crate::error::cql;

    pub type Result<T> = std::result::Result<T, cql::Error>;
}
