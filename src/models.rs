use serde::{Deserialize, Serialize};
use super::schema::cats;
#[derive(Queryable, Serialize)]
pub struct Cat {
pub id:i32,
pub name: String,
pub image_path:String,
}
pub struct IndexTemplateData{
    project_name:String,
    cats:Vec<Cat>,
}