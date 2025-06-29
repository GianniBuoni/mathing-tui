use super::*;

/// handle_requests will take the request's Params and call the appropriate
/// Param methods. In the event of an error, it will be mapped to the Response
/// and the rest of the struct will be empty.
pub async fn handle_requests(req: DbRequest, conn: &SqlitePool) -> DbResponse {
    // match refresh request
    if let RequestType::Refresh = req.req_type {
        match StoreTotal::try_refresh().await {
            Ok(_) => {
                return DbResponse::new();
            }
            Err(e) => {
                return DbResponse::new().req_type(req.req_type).error(e);
            }
        }
    }

    // match non-refresh requests
    let res: Result<DbPayload> = match req.payload {
        DbPayload::ItemParams(i) => match req.req_type {
            RequestType::GetAll => i.get_all(conn).await.map(DbPayload::Items),
            RequestType::Get => i.get(conn).await.map(DbPayload::Item),
            RequestType::Post => i.post(conn).await.map(DbPayload::Item),
            RequestType::Update => i.update(conn).await.map(DbPayload::Item),
            RequestType::Delete => {
                i.delete(conn).await.map(DbPayload::AffectedRows)
            }
            _ => {
                let req_type = format!("{:?}", req.req_type);
                Err(RequestError::unhandled("request type", req_type).into())
            }
        },
        DbPayload::UserParams(u) => match req.req_type {
            RequestType::GetAll => u.get_all(conn).await.map(DbPayload::Users),
            RequestType::Get => u.get(conn).await.map(DbPayload::User),
            RequestType::Post => u.post(conn).await.map(DbPayload::User),
            RequestType::Update => u.update(conn).await.map(DbPayload::User),
            RequestType::Delete => {
                u.delete(conn).await.map(DbPayload::AffectedRows)
            }
            _ => {
                let req_type = format!("{:?}", req.req_type);
                Err(RequestError::unhandled("request type: ", req_type).into())
            }
        },
        DbPayload::ReceiptParams(r) => match req.req_type {
            RequestType::GetAll => {
                r.get_all(conn).await.map(DbPayload::Receipts)
            }
            RequestType::Get => r.get(conn).await.map(DbPayload::Receipt),
            RequestType::Post => r.post(conn).await.map(DbPayload::Receipt),
            RequestType::Update => r.update(conn).await.map(DbPayload::Receipt),
            RequestType::Delete => {
                r.delete(conn).await.map(DbPayload::AffectedRows)
            }
            RequestType::Reset => {
                r.reset(conn).await.map(DbPayload::AffectedRows)
            }
            _ => {
                let req_type = format!("{:?}", req.req_type);
                Err(RequestError::unhandled("request type", req_type).into())
            }
        },
        _ => Err(RequestError::unhandled("payload", req.payload).into()),
    };

    match res {
        Ok(payload) => {
            DbResponse::new().req_type(req.req_type).payload(payload)
        }
        Err(e) => DbResponse::new().req_type(req.req_type).error(e),
    }
}
