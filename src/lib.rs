/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
use std::sync::Arc;
uniffi::setup_scaffolding!();

#[derive(Debug, thiserror::Error)]
#[derive(uniffi::Error)]
pub enum TelephoneError {
    #[error("Busy")]
    Busy,
    #[error("InternalTelephoneError")]
    InternalTelephoneError,
}

// Need to implement this From<> impl in order to handle unexpected callback errors.  See the
// Callback Interfaces section of the handbook for more info.
impl From<uniffi::UnexpectedUniFFICallbackError> for TelephoneError {
    fn from(_: uniffi::UnexpectedUniFFICallbackError) -> Self {
        Self::InternalTelephoneError
    }
}

// SIM cards.
#[uniffi::export(with_foreign)]
pub trait SimCard: Send + Sync {
    fn name(&self) -> String;
}

struct RustySim;
impl SimCard for RustySim {
    fn name(&self) -> String {
        "rusty!".to_string()
    }
}

// namespace functions.
#[uniffi::export]
fn get_sim_cards() -> Vec<Arc<dyn SimCard>> {
    vec![Arc::new(RustySim {})]
}

// The call-answer, callback interface.
#[uniffi::export(callback_interface)]
pub trait CallAnswerer {
    fn answer(&self) -> Result<String, TelephoneError>;
}

#[derive(Debug, Default, Clone)]
#[derive(uniffi::Object)]
pub struct Telephone;
#[uniffi::export]
impl Telephone {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {})
    }

    pub fn call(
        &self,
        // Traits are Arc<>, callbacks Box<>.
        sim: Arc<dyn SimCard>,
        answerer: Box<dyn CallAnswerer>,
    ) -> Result<String, TelephoneError> {
        if sim.name() != "rusty!" {
            Ok(format!("{} est bon marché", sim.name()))
        } else {
            answerer.answer()
        }
    }
}
