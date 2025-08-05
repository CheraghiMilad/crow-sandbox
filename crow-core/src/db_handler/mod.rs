// Defines the `JobDatabase` trait, outlining the async methods for initializing,
// connecting, querying, inserting, and updating job records.
pub mod db_interface;

// Provides the concrete PostgreSQL-backed implementation of `JobDatabase`,
// using `tokio-postgres` to fulfill the trait’s requirements.
pub mod impl_postgres;
