use diesel::pg::PgConnection;

use r2d2;
use r2d2_diesel::ConnectionManager;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

use std::env;
use std::ops::Deref;

pub type ConnMan = r2d2::Pool<ConnectionManager<PgConnection>>;
pub struct Conn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

pub fn db_url() -> String {
    return env::var("DATABASE_URL").expect("DATABASE_URL must be set");
}

/// Connect to db
pub fn connect() -> ConnMan {
    let conn_man = ConnectionManager::<PgConnection>::new(db_url());
    return r2d2::Pool::builder()
        .max_size(20)
        .build(conn_man)
        .expect("Cannot Build DB Pool");
}

impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, Self::Error> {
        let pool = request.guard::<State<ConnMan>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Conn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for Conn {
    type Target = PgConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
