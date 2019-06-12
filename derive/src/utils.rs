use syn::Type;
use syn::TypePath;

/// Return `true` if a type is an `Option` type.
pub fn is_option(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(segment) = path.segments.iter().next() {
            segment.ident == "Option"
        } else {
            false
        }
    } else {
        false
    }
}
