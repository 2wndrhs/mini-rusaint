use std::sync::Arc;

#[derive(uniffi::Object)]
pub struct USaintSession(mini_rusaint::USaintSession);

#[derive(Debug, thiserror::Error, uniffi::Error)]
#[uniffi(flat_error)]
pub enum USaintSessionError {
    #[error(transparent)]
    OriginalUSaintSessionError(#[from] mini_rusaint::USaintSessionError),
}

impl USaintSession {
    pub fn original(&self) -> mini_rusaint::USaintSession {
        self.0.clone()
    }
}

#[uniffi::export]
impl USaintSession {
    #[uniffi::constructor]
    pub async fn new(
        id: String,
        password: String,
    ) -> Result<Arc<USaintSession>, USaintSessionError> {
        let session = mini_rusaint::USaintSession::with_password(id, password).await?;

        Ok(Arc::new(USaintSession(session)))
    }
}
