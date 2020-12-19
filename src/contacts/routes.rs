use crate::contacts::Contact;
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/contacts")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let contacts = Contact::find_all()?;
    Ok(HttpResponse::Ok().json(contacts))
}

#[get("/contacts/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let contact = Contact::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(contact))
}

#[post("/contacts")]
async fn create(contact: web::Json<Contact>) -> Result<HttpResponse, CustomError> {
    let contact = Contact::create(contact.into_inner())?;
    Ok(HttpResponse::Ok().json(contact))
}

#[put("/contacts/{id}")]
async fn update(
    id: web::Path<i32>,
    contact: web::Json<Contact>,
) -> Result<HttpResponse, CustomError> {
    let contact = Contact::update(id.into_inner(), contact.into_inner())?;
    Ok(HttpResponse::Ok().json(contact))
}

#[delete("/contacts/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_contact = Contact::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_contact })))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}
