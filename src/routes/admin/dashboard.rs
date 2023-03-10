use actix_web::HttpResponse;

pub async fn admin_dashbaord() -> HttpResponse {
    HttpResponse::Ok().finish()
}
