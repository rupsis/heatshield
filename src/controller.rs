use db::establish_connection as connection;
use diesel::{
    self, expression::BoxableExpression, pg::Pg, prelude::*, result::Error, sql_types::Bool,
};

pub type Expr<T> = Box<BoxableExpression<T, Pg, SqlType = Bool>>;

pub trait ResourceController {
    type SQLType;
    type DBTable: diesel::Table;
    type Model: Insertable<Self::DBTable>;
    type ModelWithId: Queryable<Self::SQLType, Pg>;

    fn _create(&self, model: &Self::Model) -> Result<Self::ModelWithId, Error>;

    fn _get_one(&self, by: Expr<Self::DBTable>) -> Result<Self::ModelWithId, Error>;

    fn _get_all(&self, by: Expr<Self::DBTable>) -> Result<Vec<Self::ModelWithId>, Error>;

    fn _update(
        &self,
        model: &Self::Model,
        by: Expr<Self::DBTable>,
    ) -> Result<Self::ModelWithId, Error>;
}
