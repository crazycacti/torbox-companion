use crate::api::ApiError;

pub fn format_api_error(error: &ApiError) -> String {
    match error {
        ApiError::AuthenticationError => {
            "Authentication failed. Please check your API key.".to_string()
        }
        ApiError::RateLimitError => {
            "Rate limit exceeded. Please wait a moment before trying again.".to_string()
        }
        ApiError::ValidationError => {
            "Invalid request. Please check your input and try again.".to_string()
        }
        ApiError::NotFoundError => {
            "Resource not found.".to_string()
        }
        ApiError::ServerError => {
            "Server error. Please try again later.".to_string()
        }
        ApiError::NetworkError => {
            "Network error. Please check your connection and try again.".to_string()
        }
        ApiError::HttpError { status_code, message } => {
            let msg = if message.trim().is_empty() {
                match *status_code {
                    400 => "Bad request".to_string(),
                    401 => "Authentication failed".to_string(),
                    403 => "Access forbidden".to_string(),
                    404 => "Resource not found".to_string(),
                    422 => "Validation error".to_string(),
                    429 => "Rate limit exceeded".to_string(),
                    500..=599 => format!("Server error ({})", status_code),
                    _ => format!("HTTP error ({})", status_code),
                }
            } else {
                message.clone()
            };
            match *status_code {
                400 => format!("Bad request: {}", msg),
                401 => format!("Authentication failed: {}", msg),
                403 => format!("Access forbidden: {}", msg),
                404 => format!("Resource not found: {}", msg),
                422 => format!("Validation error: {}", msg),
                429 => format!("Rate limit exceeded: {}", msg),
                500..=599 => format!("Server error ({}): {}", status_code, msg),
                _ => format!("HTTP error ({}): {}", status_code, msg),
            }
        }
    }
}

pub fn get_error_summary(error: &ApiError) -> String {
    match error {
        ApiError::AuthenticationError => "Authentication failed".to_string(),
        ApiError::RateLimitError => "Rate limit exceeded".to_string(),
        ApiError::ValidationError => "Invalid request".to_string(),
        ApiError::NotFoundError => "Not found".to_string(),
        ApiError::ServerError => "Server error".to_string(),
        ApiError::NetworkError => "Network error".to_string(),
        ApiError::HttpError { status_code, .. } => {
            match *status_code {
                400 => "Bad request".to_string(),
                401 => "Authentication failed".to_string(),
                403 => "Access forbidden".to_string(),
                404 => "Not found".to_string(),
                422 => "Validation error".to_string(),
                429 => "Rate limit exceeded".to_string(),
                500..=599 => format!("Server error ({})", status_code),
                _ => format!("HTTP error ({})", status_code),
            }
        }
    }
}

