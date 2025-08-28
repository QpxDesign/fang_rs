use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Author {
    pub author_id: String,
    pub name: String,
    pub year: i64,
    pub bio: String,
    pub email: String,
    pub perm_level: i16,
    pub google_magic: String,
    pub club_position: String,
    pub formatted_name: String,
    pub formatted_year: String,
    pub rank: i16,
}
