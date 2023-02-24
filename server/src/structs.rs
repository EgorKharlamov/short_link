use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize)]
pub struct GetLinkById {
    pub link_id: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct SaveLink {
    /// Value for the `Link` need to be shorter.
    pub link: String,
}
