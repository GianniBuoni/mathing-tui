use super::*;

/// handle_requests will take the request's Params and call the appropriate
/// Param methods. In the event of an error, it will be mapped to the Response
/// and the rest of the struct will be empty.
pub async fn handle_requests(req: DbRequest, conn: &SqlitePool) -> DbResponse {
    // match non-refresh requests
    let res: Result<DbPayload> = match req.payload {
        DbPayload::ItemParams(i) => i.make_request(conn, req.req_type).await,
        DbPayload::UserParams(u) => u.make_request(conn, req.req_type).await,
        DbPayload::ReceiptParams(r) => r.make_request(conn, req.req_type).await,
        DbPayload::StoreTotal => StoreTotal::try_refresh(conn).await,
        _ => Err(RequestError::unhandled("payload", req.payload).into()),
    };

    match res {
        Ok(payload) => {
            DbResponse::new().req_type(req.req_type).payload(payload)
        }
        Err(e) => DbResponse::new().req_type(req.req_type).error(e),
    }
}
