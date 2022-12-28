/* Import macros and others */
use crate::schema::*;

/* For beeing able to serialize */
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct Hero {
    pub id: i32, 
    pub fantasy_name: String,
    pub real_name: Option<String>,
    pub spotted_photo: String,
    pub strength_level: i32,
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name = "heroes"]
pub struct NewHero<'x> {
    pub fantasy_name: &'x str,
    pub real_name: Option<&'x str>,
    pub spotted_photo: String,
    pub strength_level: i32,
}

/* #[derive(Debug)]
pub enum HeroEnum {
    Int(i32),
    String,
}

pub fn hero_enumerator() -> &Vec<Hero> {
    let id = HeroEnum::Int(i32);
    let fan = HeroEnum::String;
    let rn = HeroEnum::Option<String>;
    let sp = HeroEnum::String;
    let sl = HeroEnum::Int(i32);

    let mut herovec = vec![id, fan, rn, sp, sl];

    for hero in &herovec {

    }
} */
