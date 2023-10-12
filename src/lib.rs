use diesel::prelude::*;
use diesel::pg::Pg;

table! {
    users (id) {
        id -> Int8,
        #[max_length = 255]
        email -> Varchar,
    }
}

#[derive(Debug, Queryable, Selectable, Identifiable, QueryableByName, AsChangeset)]
#[cfg_attr(test, derive(Eq, PartialEq))]
#[diesel(table_name = users, check_for_backend(Pg))]
pub struct User {
    pub id: i64,
    pub email: String,
}

type All = diesel::dsl::Select<users::table, diesel::dsl::AsSelect<User, Pg>>;
type WithEmail<'a> = diesel::dsl::Eq<users::email, &'a str>;
type ByEmail<'a> = diesel::dsl::Filter<All, WithEmail<'a>>;


impl User {
    pub fn all() -> All {
        users::table.select(Self::as_select())
    }

    pub fn with_email(email: &str) -> WithEmail<'_> {
        users::email.eq(email)
    }

    pub fn by_email(email: &str) -> ByEmail<'_> {
        Self::all().filter(Self::with_email(email))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    }
}
