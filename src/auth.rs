use nonstick::{
    AuthnFlags, ConversationAdapter, Result as PamResult, Transaction, TransactionBuilder,
};
use std::ffi::{OsStr, OsString};

struct PasswordConvo {
    password: String,
}

impl ConversationAdapter for PasswordConvo {
    fn prompt(&self, _request: impl AsRef<OsStr>) -> PamResult<OsString> {
        Ok(OsString::from(&self.password))
    }

    fn masked_prompt(&self, _request: impl AsRef<OsStr>) -> PamResult<OsString> {
        Ok(OsString::from(&self.password))
    }

    fn error_msg(&self, message: impl AsRef<OsStr>) {
        log::error!("PAM: {}", message.as_ref().to_string_lossy());
    }

    fn info_msg(&self, message: impl AsRef<OsStr>) {
        log::debug!("PAM: {}", message.as_ref().to_string_lossy());
    }
}

pub fn verify_credentials(username: &str, password: &str) -> PamResult<()> {
    let convo = PasswordConvo { password: password.into() };
    let mut txn = TransactionBuilder::new_with_service("login")
        .username(username)
        .build(convo.into_conversation())?;
    txn.authenticate(AuthnFlags::empty())?;
    txn.account_management(AuthnFlags::empty())?;
    Ok(())
}
