use std::time::Duration;
extern crate rustc_version_runtime;
use rustc_version_runtime::version;

/// Any filters defined in `mod filters` are accessible in your template documents.
use thousands::Separable;

use crate::utils::system::get_os_info;

pub fn format_u32(n: &u32) -> ::askama::Result<String> {
    Ok(n.separate_with_commas().into())
}

pub fn elapsed_as_millis(d: &Duration) -> ::askama::Result<String> {
    Ok(d.as_millis().separate_with_commas())
}

pub fn elapsed_as_micros(d: &Duration) -> ::askama::Result<String> {
    Ok(d.as_micros().separate_with_commas())
}

pub fn elapsed_as_nanos(d: &Duration) -> ::askama::Result<String> {
    Ok(d.as_micros().separate_with_commas())
}

pub fn rustc_version(_: &str) -> ::askama::Result<String> {
    Ok(version().to_string())
} 

pub fn os_arch(_: &str) -> ::askama::Result<String> {
    Ok(get_os_info())
} 
