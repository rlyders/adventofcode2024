/// Any filters defined in `mod filters` are accessible in your template documents.
use thousands::Separable;

pub fn format_u32(n: &u32) -> ::askama::Result<String> {
    Ok(n.separate_with_commas().into())
}
