use actix_web::web::ServiceConfig;

use super::controller::signup;

pub fn init_auth_routes(cfg: &mut ServiceConfig) {
    cfg.service(signup);
}
