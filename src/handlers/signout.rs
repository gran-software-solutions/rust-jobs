use actix_web::HttpResponse;

use crate::{
    session_state::TypedSession,
    utils::{e500, see_other},
};

pub async fn signout(session: TypedSession) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(e500)?.is_none() {
        Ok(see_other("/signin"))
    } else {
        session.log_out();
        Ok(see_other("/signin"))
    }
}
