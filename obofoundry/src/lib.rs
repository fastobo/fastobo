use serde::Serialize;
use serde::Deserialize;
use serde::de::Deserializer;
use serde::de::Error;
use serde::de::Unexpected;


fn bool_from_int<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    match Option::<u8>::deserialize(deserializer)? {
        None => Ok(None),
        Some(0) => Ok(Some(false)),
        Some(1) => Ok(Some(true)),
        Some(other) => Err(Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

fn bool_true() -> bool { true }
fn bool_false() -> bool { false }



#[derive(Serialize, Deserialize)]
pub struct Foundry {
    pub ontologies: Vec<Ontology>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ontology {
    activity_status: ActivityStatus,
    #[serde(rename = "alternativePrefix", alias = "alternatePrefix")]
    alternative_prefix: Option<String>,
    biosharing: Option<String>,
    browsers: Option<Vec<Browser>>,
    pub build: Option<Build>,
    canonical: Option<String>,
    contact: Option<Contact>,
    #[serde(rename = "createdWith")]
    created_with: Option<String>,
    description: Option<String>,
    dependencies: Option<Vec<Dependency>>,
    development: Option<Development>,
    depicted_by: Option<String>,
    documentation: Option<String>,
    domain: Option<String>,
    #[serde(rename = "DO wiki")]
    do_wiki: Option<String>,
    #[serde(rename = "exampleClass")]
    example_class: Option<String>,
    facebook: Option<String>,
    funded_by: Option<Vec<String>>,
    google_plus: Option<String>,
    homepage: Option<String>,
    id: String,
    #[serde(default = "bool_true")]
    in_foundry: bool,
    in_foundry_order: Option<usize>,
    integration_server: Option<String>,
    #[serde(default = "bool_false")]
    is_obsolete: bool,
    jobs: Option<Vec<Job>>,
    label: Option<String>,
    layout: String,
    license: Option<License>,
    mailing_list: Option<String>,
    ontology_purl: Option<String>,
    page: Option<String>,
    #[serde(rename = "preferredPrefix")]
    preferred_prefix: Option<String>,
    products: Option<Vec<Product>>,
    publications: Option<Vec<Publication>>,
    redirects: Option<Vec<Redirect>>,
    releases: Option<String>,
    replaced_by: Option<String>,
    repository: Option<String>,
    source: Option<String>,
    taxon: Option<Taxon>,
    termgenie: Option<String>,
    title: String,
    #[serde(alias = "issue")]
    tracker: Option<String>,
    #[serde(rename = "type")]
    ty: Option<String>,
    twitter: Option<String>,
    #[serde(alias = "used_by")]
    pub usages: Option<Vec<Usage>>,
    validate: Option<bool>,
    #[serde(rename = "wasDerivedFrom")]
    was_derived_from: Option<String>,
    wikidata_template: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Redirect {
    #[serde(rename = "match")]
    path: String,
    url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Development {
    id_policy: String
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Dependency {
    id: String,
    title: Option<String>,
    #[serde(rename = "type")]
    ty: Option<String>,
    subset: Option<String>,
    description: Option<String>,
    connects: Option<Vec<Dependency>>,
    publications: Option<Vec<Publication>>,
}


// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "lowercase", tag = "method")]
// pub enum Build {
//
//     #[serde(default)]
//     Source {
//         source_url: String,
//     },
//
//     Archive {
//         path: String,
//         source_url: String,
//         infallible: u8,
//     },
//     Vcs {
//         checkout: String,
//         infallible: u8,
//         system: String,
//         path: Option<String>,
//     },
//     Obo2Owl {
//         source_url: String,
//     }
// }


// FIXME!
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Build {
    checkout: Option<String>,
    // #[serde(deserialize_with = "bool_from_int")]
    // infallible: Option<bool>,
    infallible: Option<u8>,
    insert_ontology_id: Option<bool>,
    pub method: Option<String>,
    notes: Option<String>,
    oort_args: Option<String>,
    path: Option<String>,
    source_url: Option<String>,
    pub system: Option<String>,
    email_cc: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct License {
    label: String,
    logo: Option<String>,
    url: String,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Contact {
    email: String,
    #[serde(alias = "contact")]
    github: Option<String>,
    label: String,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Job {
    id: String,
    #[serde(rename = "type")]
    ty: String // FIXME
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Product {
    id: String,
    is_canonical: Option<bool>,
    contact: Option<Contact>,
    connects: Option<Vec<Dependency>>,
    derived_from: Option<String>,
    description: Option<String>,
    format: Option<String>,
    homepage: Option<String>,
    license: Option<String>,
    mireots_from: Option<String>,
    ontology_purl: String,
    page: Option<String>,
    title: Option<String>,
    uses: Option<Vec<String>>,
    taxon: Option<String>,
    #[serde(rename = "type")]
    ty: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Publication {
    id: String,
    title: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Taxon {
    id: String,
    label: Option<String>
}

#[derive(Serialize, Deserialize)]
// #[serde(deny_unknown_fields)]
pub struct Usage {
    description: Option<String>,
    // examples: Option<Vec<Example>>, // FIXME: list or dict
    user: Option<String>,
    url: Option<String>,
    label: Option<String>,
    #[serde(rename = "type")]
    pub ty: Option<String>, // FIXME: enum
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Example {
    description: Option<String>,
    url: String,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "lowercase")]
pub enum ActivityStatus {
    Active,
    Inactive,
    Orphaned
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Browser {
    label: String,
    title: String,
    url: String,
}
