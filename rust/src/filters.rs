use std::{fmt::Error, time::Duration};

/// Any filters defined in `mod filters` are accessible in your template documents.
use thousands::Separable;

pub fn format_u32(n: &u32) -> ::askama::Result<String> {
    Ok(n.separate_with_commas().into())
}

pub fn elapsed_as_millis(n: &Option<Duration>) -> ::askama::Result<String> {
    match n {
        Some(d) => Ok(d.as_millis().separate_with_commas().into()),
        None => Ok("".to_string())
    }
}

pub fn elapsed_as_micros(n: &Option<Duration>) -> ::askama::Result<String> {
    match n {
        Some(d) => Ok(d.as_micros().separate_with_commas().into()),
        None => Ok("".to_string())
    }
}

pub fn elapsed_as_nanos(n: &Option<Duration>) -> ::askama::Result<String> {
    match n {
        Some(d) => Ok(d.as_nanos().separate_with_commas().into()),
        None => Ok("".to_string())
    }
}
