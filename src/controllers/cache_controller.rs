use actix_web::{HttpResponse, web};
use crate::dto::set_value_dto::SetValueDto;
use crate::common::errors::service_error::ServiceError;
use crate::services::cache_service::CacheService;

pub struct CacheController;

impl CacheController {
    pub async fn get(key: web::Path<String>) -> Result<HttpResponse, ServiceError> {
        Ok(HttpResponse::Ok().body(CacheService::get(&key)?))
    }

    pub async fn set(dto: web::Json<SetValueDto>) -> Result<HttpResponse, ServiceError> {
        CacheService::set(&dto)?;
        Ok(HttpResponse::Ok().finish())
    }

    pub async fn delete(key: web::Path<String>) -> Result<HttpResponse, ServiceError> {
        CacheService::delete(&key)?;
        Ok(HttpResponse::Ok().finish())
    }
}
