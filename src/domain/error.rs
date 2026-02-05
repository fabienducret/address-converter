use thiserror::Error;

#[derive(Debug, Error)]
pub enum AddressError {
    #[error("Champ '{field}' invalide : longueur {actual} dépasse le maximum {max}")]
    ValidationError {
        field: String,
        max: usize,
        actual: usize,
    },

    #[error("Pays invalide : '{value}'. Valeurs supportées : 'France', 'FR'")]
    InvalidCountry { value: String },

    #[error("Erreur repository ({operation}) : {source}")]
    RepositoryError {
        operation: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Erreur JSON : {0}")]
    JsonError(#[from] serde_json::Error),
}

impl From<std::io::Error> for AddressError {
    fn from(err: std::io::Error) -> Self {
        Self::RepositoryError {
            operation: String::from("fichier"),
            source: err,
        }
    }
}
