use super::*;

impl Tui {
    /// handle_requests will take the request's Params and call the appropriate
    /// Param methods. In the event of an error, it will be mapped to the Response
    /// and the rest of the struct will be empty.
    pub async fn handle_requests(req: DbRequest) -> DbResponse {
        let res: Result<DbPayload> = match get_db().await {
            Err(e) => Err(e),
            Ok(conn) => match req.payload {
                DbPayload::ItemParams(i) => match req.req_type {
                    RequestType::Get => i.get(conn).await.map(DbPayload::Item),
                    RequestType::Post => i.get(conn).await.map(DbPayload::Item),
                    RequestType::Update => {
                        i.update(conn).await.map(DbPayload::Item)
                    }
                    RequestType::Delete => {
                        i.delete(conn).await.map(DbPayload::AffectedRows)
                    }
                    _ => {
                        let msg = format!("request type: {:?}", req.req_type);
                        Err(RequestError::unhandled(msg).into())
                    }
                },
                DbPayload::UserParams(u) => match req.req_type {
                    RequestType::Get => u.get(conn).await.map(DbPayload::User),
                    RequestType::Post => u.get(conn).await.map(DbPayload::User),
                    RequestType::Update => {
                        u.update(conn).await.map(DbPayload::User)
                    }
                    RequestType::Delete => {
                        u.delete(conn).await.map(DbPayload::AffectedRows)
                    }
                    _ => {
                        let msg = format!("request type: {:?}", req.req_type);
                        Err(RequestError::unhandled(msg).into())
                    }
                },
                DbPayload::ReceiptParams(r) => match req.req_type {
                    RequestType::Get => {
                        r.get(conn).await.map(DbPayload::Receipt)
                    }
                    RequestType::Post => {
                        r.get(conn).await.map(DbPayload::Receipt)
                    }
                    RequestType::Update => {
                        r.update(conn).await.map(DbPayload::Receipt)
                    }
                    RequestType::Delete => {
                        r.delete(conn).await.map(DbPayload::AffectedRows)
                    }
                    _ => {
                        let msg = format!("request type: {:?}", req.req_type);
                        Err(RequestError::unhandled(msg).into())
                    }
                },
                _ => {
                    let msg = format!("payload: {:?}", req.payload);
                    Err(RequestError::unhandled(msg).into())
                }
            },
        };

        match res {
            Ok(payload) => {
                DbResponse::new().req_type(req.req_type).payload(payload)
            }
            Err(e) => DbResponse::new().req_type(req.req_type).error(e),
        }
    }
}
