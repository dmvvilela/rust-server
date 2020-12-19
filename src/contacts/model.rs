use crate::db;
use crate::error_handler::CustomError;
use crate::schema::contacts;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable)]
#[table_name = "contacts"]
pub struct Contact {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "contacts"]
pub struct NewContact {
    pub name: String,
    pub email: String,
    pub created_at: String,
}

impl Contact {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let contacts = contacts::table
            .order(contacts::id.asc())
            .load::<Contact>(&conn)?;
        Ok(contacts)
    }
    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let contact = contacts::table.filter(contacts::id.eq(id)).first(&conn)?;
        Ok(contact)
    }
    pub fn create(contact: Contact) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let contact = Contact::from(contact);
        let contact = diesel::insert_into(contacts::table)
            .values(&contact)
            .get_result(&conn)?;
        Ok(contact)
    }
    pub fn update(id: i32, contact: Contact) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let contact = diesel::update(contacts::table)
            .filter(contacts::id.eq(id))
            .set(contact)
            .get_result(&conn)?;
        Ok(contact)
    }
    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(contacts::table.filter(contacts::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

impl Contact {
    fn from(contact: Contact) -> NewContact {
        NewContact {
            name: contact.name,
            email: contact.email,
            created_at: "hoje".to_string(),
        }
    }
}
