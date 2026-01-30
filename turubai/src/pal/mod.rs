#[cfg(feature = "apple")]
mod apple;

use crate::{Application, Backend};

pub fn takeover(app: Box<dyn Application>) -> ! {
    #[cfg(feature = "apple")]
    {
        apple::Context::takeover(app)
    }

    #[cfg(not(feature = "apple"))]
    panic!("No platform backend enabled. Enable the 'apple' feature on macOS.")
}

pub trait API {
    const VARIANT: Backend;

    type Context: DynContext;
}

pub trait DynContext {
    type A: API;
    fn takeover(app: Box<dyn Application>) -> !;
}

pub trait DynWindow {}

pub trait DynText {}
