use fastobo::ast as obo;
use horned_owl::model as owl;

use super::Context;
use super::IntoOwlCtx;


/// Convert a `PrefixedIdent` to an IRI using its IDspace or a default one.
impl IntoOwlCtx for obo::PrefixedIdent {
    type Owl = owl::IRI;
    fn into_owl(self, ctx: &mut Context) -> Self::Owl {
        let iri = match ctx.idspaces.get(&self.prefix) {
            Some(url) => format!("{}{}", url, self.local.as_str()),
            None => format!("{}{}{}",
                crate::constants::uri::OBO,
                self.prefix.as_str(),
                self.local.as_str()
            ),
        };
        ctx.build.iri(iri)
    }
}

/// Convert an `UnprefixedIdent` to an
impl IntoOwlCtx for obo::UnprefixedIdent {
    type Owl = owl::IRI;
    fn into_owl(self, ctx: &mut Context) -> Self::Owl {
        ctx.build.iri(format!("{}#{}", &ctx.ontology_iri, self.as_str()))
    }
}

/// Convert an OBO URL identifier to an OWL IRI.
impl IntoOwlCtx for obo::Url {
    type Owl = owl::IRI;
    fn into_owl(self, ctx: &mut Context) -> Self::Owl {
        ctx.build.iri(self.into_string())
    }
}

/// Convert an arbitrary OBO identifier to an OWL IRI.
impl IntoOwlCtx for obo::Ident {
    type Owl = owl::IRI;
    fn into_owl(self, ctx: &mut Context) -> Self::Owl {
        match self {
            obo::Ident::Url(url) => url.into_owl(ctx),
            obo::Ident::Unprefixed(id) => id.into_owl(ctx),
            obo::Ident::Prefixed(id) => id.into_owl(ctx),
        }
    }
}

/// Convert a class identifier to an OWL IRI.
impl IntoOwlCtx for obo::ClassIdent {
    type Owl = owl::IRI;
    fn into_owl(self, ctx: &mut Context) -> Self::Owl {
        obo::Ident::from(self).into_owl(ctx)
    }
}

/// Convert a subset identifier to an OWL IRI.
impl IntoOwlCtx for obo::SubsetIdent {
    type Owl = owl::IRI;
    fn into_owl(self, ctx: &mut Context) -> Self::Owl {
        obo::Ident::from(self).into_owl(ctx)
    }
}
