pub mod api_doc {
    use crate::controller as Links;
    use crate::structs::SaveLink;
    use utoipa::OpenApi;

    #[derive(OpenApi)]
    #[openapi(
    paths(
    Links::get_link_by_id,
    Links::save_link,
    ),
    components(
    schemas(Links::ResponseOkJson, Links::ResponseErrJson, SaveLink)
    ),
    tags(
    (name = "link", description = "Link shorter.")
    ),
    )]
    struct ApiDoc;

    pub fn open_api() -> utoipa::openapi::OpenApi {
        ApiDoc::openapi()
    }
}
