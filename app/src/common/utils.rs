use thiserror::Error;

// Helper function to generate the URL based on the mode
#[must_use]
pub fn make_url<T: AsRef<str>>(base_path: T, query_string: &str) -> String {
    let root = "http://localhost:3000";

    if !query_string.is_empty() {
        format!("{}{}?{}", root, base_path.as_ref(), query_string)
    } else {
        format!("{}{}", root, base_path.as_ref())
    }
}

#[derive(Clone, Error, Debug, PartialEq, Eq)]
pub enum GraphErr {
    #[error("{0}")]
    Custom(String),
}

/// Helper to convert an input str into a sanitized `id` suitable for HTML node
/// identifiers.
pub fn sanitize_id(input: &str) -> String {
    input.to_lowercase().replace(' ', "-")
}

pub fn get_initials(user_name: &str) -> String {
    let mut parts = user_name.split_whitespace();
    let first_initial = parts.next().and_then(|first| {
        first
            .chars()
            .next()
            .map(|c| c.to_uppercase().next().unwrap_or(c))
    });
    let last_initial = parts.next().and_then(|last| {
        last.chars()
            .next()
            .map(|c| c.to_uppercase().next().unwrap_or(c))
    });

    match (first_initial, last_initial) {
        (Some(f), Some(l)) => format!("{}{}", f, l),
        (Some(f), None) => format!("{}", f),
        _ => "VZ".to_string(),
    }
}

/// Helper to determine if the password is complex enough
pub fn is_password_complex(password: &str) -> bool {
    password.len() >= 16
}
