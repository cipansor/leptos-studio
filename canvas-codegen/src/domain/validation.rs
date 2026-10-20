use super::error::ValidationError;
use regex::Regex;

pub trait Validator<T> {
    fn validate(&self, value: &T) -> Result<(), ValidationError>;
}

/// Component name validator
/// Valid names must:
/// - Not be empty
/// - Be valid Rust identifiers (alphanumeric + underscore)
/// - Not start with a digit
pub struct ComponentNameValidator;

impl Validator<String> for ComponentNameValidator {
    fn validate(&self, name: &String) -> Result<(), ValidationError> {
        if name.trim().is_empty() {
            return Err(ValidationError::EmptyName);
        }

        // Check if it's a valid Rust identifier
        let re = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
        if !re.is_match(name) {
            return Err(ValidationError::InvalidName(name.clone()));
        }

        Ok(())
    }
}

/// HTML template validator
/// Valid templates must:
/// - Not be empty
/// - Have minimum length of 3 characters
/// - Contain at least one HTML tag
/// - Not contain dangerous patterns (XSS prevention)
pub struct HtmlTemplateValidator;

impl Validator<String> for HtmlTemplateValidator {
    fn validate(&self, template: &String) -> Result<(), ValidationError> {
        if template.trim().is_empty() {
            return Err(ValidationError::EmptyTemplate);
        }

        if template.len() < 3 {
            return Err(ValidationError::InvalidTemplate(
                "Template too short".to_string(),
            ));
        }

        // Check if contains HTML tags
        let tag_re = Regex::new(r"<[^>]+>").unwrap();
        if !tag_re.is_match(template) {
            return Err(ValidationError::InvalidTemplate(
                "Template must contain at least one HTML tag".to_string(),
            ));
        }

        // Security: Check for dangerous patterns
        if !crate::utils::sanitize::is_html_safe(template) {
            return Err(ValidationError::InvalidTemplate(
                "Template contains potentially dangerous content (scripts, event handlers, or dangerous protocols)".to_string(),
            ));
        }

        Ok(())
    }
}

/// Sanitizing HTML template validator
/// Same as HtmlTemplateValidator but also sanitizes the input
pub struct SanitizingHtmlTemplateValidator;

impl SanitizingHtmlTemplateValidator {
    /// Validate and sanitize the template, returning the sanitized version
    pub fn validate_and_sanitize(&self, template: &str) -> Result<String, ValidationError> {
        if template.trim().is_empty() {
            return Err(ValidationError::EmptyTemplate);
        }

        if template.len() < 3 {
            return Err(ValidationError::InvalidTemplate(
                "Template too short".to_string(),
            ));
        }

        // Check if contains HTML tags
        let tag_re = Regex::new(r"<[^>]+>").unwrap();
        if !tag_re.is_match(template) {
            return Err(ValidationError::InvalidTemplate(
                "Template must contain at least one HTML tag".to_string(),
            ));
        }

        // Sanitize the template
        let result = crate::utils::sanitize::sanitize_html(
            template,
            &crate::utils::sanitize::SanitizeConfig::default(),
        );

        if result.was_truncated {
            return Err(ValidationError::InvalidTemplate(
                "Template is too long".to_string(),
            ));
        }

        Ok(result.html)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_name_validator_valid() {
        let validator = ComponentNameValidator;
        assert!(validator.validate(&"MyComponent".to_string()).is_ok());
        assert!(validator.validate(&"my_component".to_string()).is_ok());
        assert!(validator.validate(&"Component123".to_string()).is_ok());
        assert!(validator.validate(&"_component".to_string()).is_ok());
    }

    #[test]
    fn test_component_name_validator_empty() {
        let validator = ComponentNameValidator;
        assert!(matches!(
            validator.validate(&"".to_string()),
            Err(ValidationError::EmptyName)
        ));
        assert!(matches!(
            validator.validate(&"   ".to_string()),
            Err(ValidationError::EmptyName)
        ));
    }

    #[test]
    fn test_component_name_validator_invalid() {
        let validator = ComponentNameValidator;
        // Starts with digit
        assert!(matches!(
            validator.validate(&"123Component".to_string()),
            Err(ValidationError::InvalidName(_))
        ));
        // Contains special characters
        assert!(matches!(
            validator.validate(&"My-Component".to_string()),
            Err(ValidationError::InvalidName(_))
        ));
        assert!(matches!(
            validator.validate(&"My Component".to_string()),
            Err(ValidationError::InvalidName(_))
        ));
    }

    #[test]
    fn test_html_template_validator_valid() {
        let validator = HtmlTemplateValidator;
        assert!(validator.validate(&"<div>Hello</div>".to_string()).is_ok());
        assert!(validator.validate(&"<p>Text</p>".to_string()).is_ok());
        assert!(
            validator
                .validate(&"<button>Click</button>".to_string())
                .is_ok()
        );
    }

    #[test]
    fn test_html_template_validator_empty() {
        let validator = HtmlTemplateValidator;
        assert!(matches!(
            validator.validate(&"".to_string()),
            Err(ValidationError::EmptyTemplate)
        ));
    }

    #[test]
    fn test_html_template_validator_invalid() {
        let validator = HtmlTemplateValidator;
        // Too short
        assert!(matches!(
            validator.validate(&"ab".to_string()),
            Err(ValidationError::InvalidTemplate(_))
        ));
        // No HTML tags
        assert!(matches!(
            validator.validate(&"Just plain text".to_string()),
            Err(ValidationError::InvalidTemplate(_))
        ));
    }
}
