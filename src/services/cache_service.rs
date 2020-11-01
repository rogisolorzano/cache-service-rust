use crate::common::errors::service_error::ServiceError;
use crate::dto::set_value_dto::SetValueDto;

pub struct CacheService;

impl CacheService {
    pub fn get(_key: &str) -> Result<String, ServiceError> {
        // TODO: Implement
        return Ok(String::from("Success!"));
    }

    pub fn set(_dto: &SetValueDto) -> Result<(), ServiceError> {
        // TODO: Implement
        Ok(())
    }

    pub fn delete(_key: &str) -> Result<(), ServiceError> {
        // TODO: Implement
        Ok(())
    }
}
