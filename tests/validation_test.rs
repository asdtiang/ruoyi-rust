use ruoyi_rust::web::extractors::validator::{ValidationError, ValidatedForm};
use axum::http::StatusCode;
use validator::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Validate)]
struct TestDto {
    #[validate(length(min = 1, max = 100))]
    name: String,
    
    #[validate(email)]
    email: String,
    
    #[validate(range(min = 18, max = 120))]
    age: u32,
}

#[test]
fn test_validation_error_display() {
    let error = ValidationError::JsonError("test error".to_string());
    let display = format!("{}", error);
    assert!(display.contains("Invalid JSON data"));
    assert!(display.contains("test error"));
}

#[test]
fn test_validation_error_form_error_display() {
    let error = ValidationError::FormError;
    let display = format!("{}", error);
    assert_eq!(display, "Invalid form data");
}

#[test]
fn test_validation_error_data_missing_display() {
    let error = ValidationError::DataMissing;
    let display = format!("{}", error);
    assert_eq!(display, "Data is missing");
}

#[test]
fn test_validation_error_debug() {
    let error = ValidationError::JsonError("test".to_string());
    let debug = format!("{:?}", error);
    assert!(debug.contains("JsonError"));
}

#[test]
fn test_testdto_valid_data() {
    let dto = TestDto {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        age: 25,
    };
    assert!(dto.validate().is_ok());
}

#[test]
fn test_testdto_invalid_name_too_short() {
    let dto = TestDto {
        name: "".to_string(),
        email: "john@example.com".to_string(),
        age: 25,
    };
    let result = dto.validate();
    assert!(result.is_err());
}

#[test]
fn test_testdto_invalid_name_too_long() {
    let dto = TestDto {
        name: "a".repeat(101),
        email: "john@example.com".to_string(),
        age: 25,
    };
    let result = dto.validate();
    assert!(result.is_err());
}

#[test]
fn test_testdto_invalid_email() {
    let dto = TestDto {
        name: "John Doe".to_string(),
        email: "invalid-email".to_string(),
        age: 25,
    };
    let result = dto.validate();
    assert!(result.is_err());
}

#[test]
fn test_testdto_invalid_age_too_young() {
    let dto = TestDto {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        age: 17,
    };
    let result = dto.validate();
    assert!(result.is_err());
}

#[test]
fn test_testdto_invalid_age_too_old() {
    let dto = TestDto {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        age: 121,
    };
    let result = dto.validate();
    assert!(result.is_err());
}

#[test]
fn test_testdto_valid_boundary_age_min() {
    let dto = TestDto {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        age: 18,
    };
    assert!(dto.validate().is_ok());
}

#[test]
fn test_testdto_valid_boundary_age_max() {
    let dto = TestDto {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        age: 120,
    };
    assert!(dto.validate().is_ok());
}

#[test]
fn test_testdto_boundary_name_min() {
    let dto = TestDto {
        name: "a".to_string(),
        email: "john@example.com".to_string(),
        age: 25,
    };
    assert!(dto.validate().is_ok());
}

#[test]
fn test_testdto_boundary_name_max() {
    let dto = TestDto {
        name: "a".repeat(100),
        email: "john@example.com".to_string(),
        age: 25,
    };
    assert!(dto.validate().is_ok());
}

#[test]
fn test_validation_error_from_validation_errors() {
    let dto = TestDto {
        name: "".to_string(),
        email: "invalid".to_string(),
        age: 10,
    };
    let validation_result = dto.validate();
    assert!(validation_result.is_err());
    
    let validation_error = ValidationError::from(validation_result.unwrap_err());
    match validation_error {
        ValidationError::Validation(_) => {},
        _ => panic!("Expected ValidationError::Validation"),
    }
}

#[test]
fn test_testdto_empty_email() {
    let dto = TestDto {
        name: "John Doe".to_string(),
        email: "".to_string(),
        age: 25,
    };
    let result = dto.validate();
    assert!(result.is_err());
}

#[test]
fn test_testdto_unicode_name() {
    let dto = TestDto {
        name: "张三".to_string(),
        email: "john@example.com".to_string(),
        age: 25,
    };
    assert!(dto.validate().is_ok());
}

#[test]
fn test_testdto_special_characters_in_name() {
    let dto = TestDto {
        name: "John-O'Brien".to_string(),
        email: "john@example.com".to_string(),
        age: 25,
    };
    assert!(dto.validate().is_ok());
}

#[test]
fn test_testdto_valid_email_formats() {
    let valid_emails = vec![
        "test@example.com",
        "user.name@domain.co.uk",
        "test+tag@example.org",
        "user123@test-domain.com",
    ];
    
    for email in valid_emails {
        let dto = TestDto {
            name: "John".to_string(),
            email: email.to_string(),
            age: 25,
        };
        assert!(dto.validate().is_ok(), "Should be valid: {}", email);
    }
}

#[test]
fn test_testdto_invalid_email_formats() {
    let invalid_emails = vec![
        "plainaddress",
        "@missinglocal.com",
        "username@.com",
        "username@",
        "username@.com",
    ];
    
    for email in invalid_emails {
        let dto = TestDto {
            name: "John".to_string(),
            email: email.to_string(),
            age: 25,
        };
        assert!(dto.validate().is_err(), "Should be invalid: {}", email);
    }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
struct OptionalDto {
    #[validate(length(min = 1))]
    optional_field: Option<String>,
}

#[test]
fn test_optional_field_none() {
    let dto = OptionalDto {
        optional_field: None,
    };
    // validator treats None as valid (no validation applied)
    assert!(dto.validate().is_ok());
}

#[test]
fn test_optional_field_some_valid() {
    let dto = OptionalDto {
        optional_field: Some("valid".to_string()),
    };
    assert!(dto.validate().is_ok());
}

#[test]
fn test_optional_field_some_invalid() {
    let dto = OptionalDto {
        optional_field: Some("".to_string()),
    };
    assert!(dto.validate().is_err());
}
