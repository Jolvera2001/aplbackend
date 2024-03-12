use surrealdb::Error;

pub trait CrudOperations<T> {
    async fn create(&self, item: T) -> Result<(), Error>;
    async fn read(&self, id: String) -> Result<Option<T>, Error>;
    async fn update(&self, id: String, item: T) -> Result<(), Error>;
    async fn delete(&self, id: String) -> Result<(), Error>;
}