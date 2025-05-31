//! Interpolates `{key}` placeholders in i18n templates.

/// Replaces `{key}` placeholders in the template with given values.
pub fn format_i18n(template: &str, vars: &[(&str, String)]) -> String {
    let mut result = template.to_string();

    // Replace each {key} with its value
    for (key, val) in vars {
        result = result.replace(&format!("{{{}}}", key), val);
    }

    result
}
