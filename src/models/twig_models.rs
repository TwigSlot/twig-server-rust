use crate::models::types::VertexKey;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use palette::Srgb;
use url::Url;

pub struct BaseDbObject {
    pub id: VertexKey,
    pub creation_datetime: DateTime<Utc>,
    pub last_modification_datetime: DateTime<Utc>
}

pub struct BaseTwigObject {
    pub db_object: BaseDbObject,
    pub name: String,
    pub description: String,
}

pub struct Tag {
    pub twig_object: BaseTwigObject,
    pub color: Srgb
}

pub struct Project<'a> {
    pub twig_object: BaseTwigObject,
    pub owner: Uuid,
    pub tags: Vec<&'a Tag>
}

pub struct Resource {
    pub twig_object: BaseTwigObject,
    pub url: Url
}
