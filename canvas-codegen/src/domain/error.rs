use thiserror::Error;

/// Error code for tracking and documentation purposes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    // Validation errors (1xxx)
    ValidationEmptyName = 1001,
    ValidationInvalidName = 1002,
    ValidationDuplicateName = 1003,
    ValidationEmptyTemplate = 1004,
    ValidationInvalidTemplate = 1005,
    ValidationMissingProperty = 1006,
    ValidationInvalidProperty = 1007,

    // Serialization errors (2xxx)
    SerializationFailed = 2001,
    DeserializationFailed = 2002,

    // Storage errors (3xxx)
    StorageNotAvailable = 3001,
    StorageReadFailed = 3002,
    StorageWriteFailed = 3003,
    StorageNoData = 3004,

    // Git errors (4xxx)
    GitOperationFailed = 4001,
    GitNotInitialized = 4002,

    // Component errors (5xxx)
    ComponentNotFound = 5001,
    ComponentLimitExceeded = 5002,
    ComponentNestingTooDeep = 5003,

    // Export errors (6xxx)
    ExportFailed = 6001,
    ExportFormatUnsupported = 6002,

    // Network errors (7xxx)
    NetworkFailed = 7001,

    // General errors (9xxx)
    Unknown = 9999,
}

impl ErrorCode {
    /// Get error code as string for display
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorCode::ValidationEmptyName => "E1001",
            ErrorCode::ValidationInvalidName => "E1002",
            ErrorCode::ValidationDuplicateName => "E1003",
            ErrorCode::ValidationEmptyTemplate => "E1004",
            ErrorCode::ValidationInvalidTemplate => "E1005",
            ErrorCode::ValidationMissingProperty => "E1006",
            ErrorCode::ValidationInvalidProperty => "E1007",
            ErrorCode::SerializationFailed => "E2001",
            ErrorCode::DeserializationFailed => "E2002",
            ErrorCode::StorageNotAvailable => "E3001",
            ErrorCode::StorageReadFailed => "E3002",
            ErrorCode::StorageWriteFailed => "E3003",
            ErrorCode::StorageNoData => "E3004",
            ErrorCode::GitOperationFailed => "E4001",
            ErrorCode::GitNotInitialized => "E4002",
            ErrorCode::ComponentNotFound => "E5001",
            ErrorCode::ComponentLimitExceeded => "E5002",
            ErrorCode::ComponentNestingTooDeep => "E5003",
            ErrorCode::ExportFailed => "E6001",
            ErrorCode::ExportFormatUnsupported => "E6002",
            ErrorCode::NetworkFailed => "E7001",
            ErrorCode::Unknown => "E9999",
        }
    }

    /// Get documentation URL for this error
    pub fn docs_url(&self) -> String {
        format!(
            "https://github.com/analisaperlengkapan/leptos-studio/wiki/Errors#{}",
            self.as_str().to_lowercase()
        )
    }
}

#[derive(Error, Debug, Clone)]
pub enum ValidationError {
    #[error("Component name is empty")]
    EmptyName,

    #[error("Component name '{0}' is invalid: must be a valid Rust identifier")]
    InvalidName(String),

    #[error("Component name '{0}' already exists")]
    DuplicateName(String),

    #[error("Template is empty")]
    EmptyTemplate,

    #[error("Template is invalid: {0}")]
    InvalidTemplate(String),

    #[error("Property '{0}' is required but not provided")]
    MissingRequiredProperty(String),

    #[error("Property '{0}' has invalid value: {1}")]
    InvalidPropertyValue(String, String),

    #[error("{0}")]
    Generic(String),
}

impl ValidationError {
    /// Get the error code for this validation error
    pub fn error_code(&self) -> ErrorCode {
        match self {
            ValidationError::EmptyName => ErrorCode::ValidationEmptyName,
            ValidationError::InvalidName(_) => ErrorCode::ValidationInvalidName,
            ValidationError::DuplicateName(_) => ErrorCode::ValidationDuplicateName,
            ValidationError::EmptyTemplate => ErrorCode::ValidationEmptyTemplate,
            ValidationError::InvalidTemplate(_) => ErrorCode::ValidationInvalidTemplate,
            ValidationError::MissingRequiredProperty(_) => ErrorCode::ValidationMissingProperty,
            ValidationError::InvalidPropertyValue(_, _) => ErrorCode::ValidationInvalidProperty,
            ValidationError::Generic(_) => ErrorCode::ValidationInvalidProperty, // Map generic to invalid property or similar
        }
    }

    /// Get user-friendly error message
    pub fn user_message(&self) -> String {
        let code = self.error_code();
        format!("[{}] {}", code.as_str(), self)
    }
}

#[derive(Error, Debug, Clone)]
pub enum AppError {
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Git error: {0}")]
    Git(String),

    #[error("Component not found: {0}")]
    ComponentNotFound(String),

    #[error("Component limit exceeded: maximum {0} components allowed")]
    ComponentLimitExceeded(usize),

    #[error("Container nesting too deep: maximum {0} levels allowed")]
    NestingTooDeep(usize),

    #[error("Export error: {0}")]
    Export(String),

    #[error("Network error: {0}")]
    Network(String),
}

pub type AppResult<T> = Result<T, AppError>;

impl AppError {
    /// Get the error code for this error
    pub fn error_code(&self) -> ErrorCode {
        match self {
            AppError::Validation(e) => e.error_code(),
            AppError::Serialization(_) => ErrorCode::SerializationFailed,
            AppError::Storage(_) => ErrorCode::StorageWriteFailed,
            AppError::Git(_) => ErrorCode::GitOperationFailed,
            AppError::ComponentNotFound(_) => ErrorCode::ComponentNotFound,
            AppError::ComponentLimitExceeded(_) => ErrorCode::ComponentLimitExceeded,
            AppError::NestingTooDeep(_) => ErrorCode::ComponentNestingTooDeep,
            AppError::Export(_) => ErrorCode::ExportFailed,
            AppError::Network(_) => ErrorCode::NetworkFailed,
        }
    }

    /// Convert error to user-friendly message with error code
    pub fn user_message(&self) -> String {
        let code = self.error_code();
        format!("[{}] {}", code.as_str(), self.display_message())
    }

    /// Get display message without error code
    fn display_message(&self) -> String {
        match self {
            AppError::Validation(e) => format!("Validation failed: {}", e),
            AppError::Serialization(msg) => format!("Failed to process data: {}", msg),
            AppError::Storage(msg) => format!("Storage error: {}", msg),
            AppError::Git(msg) => format!("Git operation failed: {}", msg),
            AppError::ComponentNotFound(name) => format!("Component '{}' not found", name),
            AppError::ComponentLimitExceeded(limit) => {
                format!("Too many components. Maximum allowed: {}", limit)
            }
            AppError::NestingTooDeep(limit) => {
                format!("Container nesting too deep. Maximum depth: {}", limit)
            }
            AppError::Export(msg) => format!("Export failed: {}", msg),
            AppError::Network(msg) => format!("Network error: {}", msg),
        }
    }

    /// Get help text for this error
    pub fn help_text(&self) -> Option<&'static str> {
        match self {
            AppError::Validation(ValidationError::EmptyName) => {
                Some("Component names cannot be empty. Please provide a valid identifier.")
            }
            AppError::Validation(ValidationError::InvalidName(_)) => Some(
                "Component names must start with a letter or underscore, and contain only letters, numbers, and underscores.",
            ),
            AppError::Validation(ValidationError::InvalidTemplate(_)) => Some(
                "Templates must contain valid HTML and should not include script tags or event handlers for security.",
            ),
            AppError::Storage(_) => Some(
                "Check if localStorage is available and not full. Try clearing browser cache if the issue persists.",
            ),
            AppError::ComponentLimitExceeded(_) => {
                Some("Consider grouping components into containers or removing unused components.")
            }
            AppError::NestingTooDeep(_) => {
                Some("Try flattening your component hierarchy or using fewer nested containers.")
            }
            _ => None,
        }
    }

    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            AppError::Validation(_) => true,
            AppError::Serialization(_) => false,
            AppError::Storage(_) => true,
            AppError::Git(_) => true,
            AppError::ComponentNotFound(_) => true,
            AppError::ComponentLimitExceeded(_) => true,
            AppError::NestingTooDeep(_) => true,
            AppError::Export(_) => true,
            AppError::Network(_) => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_codes_are_unique() {
        // Ensure error codes don't overlap
        let codes = [
            ErrorCode::ValidationEmptyName,
            ErrorCode::ValidationInvalidName,
            ErrorCode::SerializationFailed,
            ErrorCode::StorageNotAvailable,
            ErrorCode::ComponentNotFound,
            ErrorCode::ExportFailed,
        ];

        for (i, code1) in codes.iter().enumerate() {
            for code2 in codes.iter().skip(i + 1) {
                assert_ne!(*code1 as u32, *code2 as u32, "Error codes must be unique");
            }
        }
    }

    #[test]
    fn test_user_message_includes_code() {
        let error = AppError::ComponentNotFound("MyButton".to_string());
        let message = error.user_message();
        assert!(message.contains("[E5001]"));
        assert!(message.contains("MyButton"));
    }

    #[test]
    fn test_validation_error_code() {
        let error = ValidationError::EmptyName;
        assert_eq!(error.error_code(), ErrorCode::ValidationEmptyName);
    }

    #[test]
    fn test_error_is_recoverable() {
        assert!(AppError::Validation(ValidationError::EmptyName).is_recoverable());
        assert!(!AppError::Serialization("parse error".to_string()).is_recoverable());
    }

    #[test]
    fn test_help_text_available() {
        let error = AppError::ComponentLimitExceeded(1000);
        assert!(error.help_text().is_some());
    }
}
