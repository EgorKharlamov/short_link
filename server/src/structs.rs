use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetLinkById {
    pub link_id: String,
}

#[derive(Deserialize)]
pub struct SaveLink {
    pub link: String,
}
