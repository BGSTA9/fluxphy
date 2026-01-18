//! Domain-Aware Validation Plugins for FluxPhy
//!
//! Provides an interface for validating file content integrity and format compliance
//! beyond simple checksums.

use crate::error::FluxResult;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResult {
    Valid,
    Invalid(String),
    Skipped(String),
}

/// Trait for domain-specific validators
pub trait Validator {
    /// Name of the validator
    fn name(&self) -> &str;
    
    /// Validate the file at the given path
    fn validate(&self, path: &Path) -> FluxResult<ValidationResult>;
}

/// A basic validator that checks file signatures (magic bytes)
pub struct MagicBytesValidator;

impl Validator for MagicBytesValidator {
    fn name(&self) -> &str {
        "Magic Bytes Check"
    }

    fn validate(&self, path: &Path) -> FluxResult<ValidationResult> {
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(_) => return Ok(ValidationResult::Skipped("Could not open file".to_string())),
        };

        // Read first 8 bytes
        let mut buffer = [0u8; 8];
        let bytes_read = match file.read(&mut buffer) {
            Ok(n) => n,
            Err(_) => return Ok(ValidationResult::Skipped("Read error".to_string())),
        };

        if bytes_read < 4 {
             return Ok(ValidationResult::Skipped("File too small".to_string()));
        }

        // Check common signatures
        // PNG
        if buffer.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
            return Ok(ValidationResult::Valid);
        }
        // JPEG (approximate)
        if buffer.starts_with(&[0xFF, 0xD8, 0xFF]) {
             return Ok(ValidationResult::Valid);
        }
        // PDF
        if buffer.starts_with(b"%PDF") {
             return Ok(ValidationResult::Valid);
        }
        // GZIP
        if buffer.starts_with(&[0x1F, 0x8B]) {
             return Ok(ValidationResult::Valid);
        }
        
        // If we don't recognize it, we don't necessarily fail it unless we enforce types.
        // For general transfer, this validator just identifies known types.
        // But to be "Validation", maybe it should fail if extension implies a type?
        // Let's implement extension matching.
        
        let ext = path.extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_default();
            
        match ext.as_str() {
            "png" => {
                if !buffer.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
                    return Ok(ValidationResult::Invalid("Invalid PNG header".to_string()));
                }
            },
            "jpg" | "jpeg" => {
                 if !buffer.starts_with(&[0xFF, 0xD8, 0xFF]) {
                    return Ok(ValidationResult::Invalid("Invalid JPEG header".to_string()));
                }
            },
            "gz" => {
                 if !buffer.starts_with(&[0x1F, 0x8B]) {
                    return Ok(ValidationResult::Invalid("Invalid GZIP header".to_string()));
                }
            },
            "pdf" => {
                 if !buffer.starts_with(b"%PDF") {
                    return Ok(ValidationResult::Invalid("Invalid PDF header".to_string()));
                }
            }
            _ => return Ok(ValidationResult::Skipped("Unknown file type".to_string())),
        }

        Ok(ValidationResult::Valid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_magic_bytes_validator() {
        let mut file = NamedTempFile::new().unwrap();
        // Write PNG header
        file.write_all(&[0x89, 0x50, 0x4E, 0x47]).unwrap();
        
        let validator = MagicBytesValidator;
        assert_eq!(validator.validate(file.path()).unwrap(), ValidationResult::Valid);
    }
}
