use actix_files::Files;
use actix_web::{http, web, App, Error, HttpResponse, HttpServer,};
use handlebars::Handlebars;
use diesel::pg::PgConnection;
use diesel::{prelude::*, connection};
use diesel::r2d2::{self, ConnectionManager};
mod models;
mod schema;
use self::models::*
use self::schema::cats::dsl::*;

async fn index(
    hb:web::Data<Handlebars<_>>,
    pool:Data<Dbpool>,
) -> Result<HttpResponse,Error>{
    let connection =pool.get()
    .expect("can't get db connection from pool");
    let cats_data = wep::block(move||{
        cats.limit(100).load::<Cat>(&connection)
    })
.await
.map_err(|_|HttpResponse::InternalServerError().finish())?; 
let data =IndexTemplateData{
    project_name:"catdex".to_string(),
    cats:cats_data,
};
let body =hb.render("index",&data).unwrap();
Ok(HttpResponse::Ok().body(body))
}    







#[actix_web::main]
async fn main() ->std::io::Result<()> {
}