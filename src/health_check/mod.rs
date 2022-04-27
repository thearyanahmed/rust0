use actix_web::{Scope, web};
use crate::health_check::routes::health_check;

mod routes;

pub fn register() -> Scope {
	web::scope("/api/v1").service(
		web::resource("/health-check")
			.route(web::get().to(health_check))
	)
}