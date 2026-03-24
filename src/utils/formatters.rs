/// For Plain T
pub trait Pipe {
    fn text(&self, fallback: Option<&str>) -> Option<String>;
    fn float(&self, precision: Option<usize>, fallback: Option<&str>) -> Option<String>;
}

/// For Option<T>
pub trait PipeOption {
    fn text(&self, fallback: Option<&str>) -> Option<String>;
    fn float(&self, precision: Option<usize>, fallback: Option<&str>) -> Option<String>;
}

/// For Option<T>
impl<T: std::fmt::Display> PipeOption for Option<T> {
    /// Formats any value that can be displayed as a string.
    ///
    /// Handles Option<String>, Option<&str>, Option<i32>, Option<u32>, etc.
    ///
    /// Falls back to provided default or "N/A" if None or empty.
    fn text(&self, fallback: Option<&str>) -> Option<String> {
        match &self {
            Some(v) => {
                let s = v.to_string();
                if s.trim().is_empty() {
                    Some(fallback.unwrap_or("N/A").to_string())
                } else {
                    Some(s)
                }
            }
            None => Some(fallback.unwrap_or("N/A").to_string()),
        }
    }

    /// Formats a float value from anything parseable as f64 (including strings from APIs).
    ///
    /// Optional precision controls decimal places.
    ///
    /// Falls back to provided default or "N/A" if None, empty, or unparseable.
    fn float(&self, precision: Option<usize>, fallback: Option<&str>) -> Option<String> {
        match &self {
            Some(v) => match v.to_string().trim().parse::<f64>() {
                Ok(f) => Some(match precision {
                    Some(p) => format!("{:.prec$}", f, prec = p),
                    None => {
                        if f.fract() == 0.0 {
                            format!("{}", f as i64)
                        } else {
                            f.to_string()
                        }
                    }
                }),
                Err(_) => Some(fallback.unwrap_or("N/A").to_string()),
            },
            None => Some(fallback.unwrap_or("N/A").to_string()),
        }
    }
}

/// For plain T — wraps in Some and delegates
impl<T: std::fmt::Display> Pipe for T {
    /// Formats any value that can be displayed as a string.
    ///
    /// Handles Option<String>, Option<&str>, Option<i32>, Option<u32>, etc.
    ///
    /// Falls back to provided default or "N/A" if None or empty.
    fn text(&self, fallback: Option<&str>) -> Option<String> {
        Some(&self).text(fallback)
    }

    /// Formats a float value from anything parseable as f64 (including strings from APIs).
    ///
    /// Optional precision controls decimal places.
    ///
    /// Falls back to provided default or "N/A" if None, empty, or unparseable.
    fn float(&self, precision: Option<usize>, fallback: Option<&str>) -> Option<String> {
        Some(&self).float(precision, fallback)
    }
}
