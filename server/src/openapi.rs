pub mod api_doc {
    use crate::controller;
    use utoipa::OpenApi;

    #[derive(OpenApi)]
    #[openapi(
    paths(
    controller::get_link_by_id,
    controller::save_link,
    ),
    components(
    schemas(controller::ResponseOkJson, controller::ResponseErrJson)
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
