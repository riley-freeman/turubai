#[cfg(feature = "apple")]
mod apple;

#[cfg(feature = "gtk")]
mod gtk;

use crate::{Application, Backend};

pub fn takeover(app: Box<dyn Application>) -> ! {
    #[cfg(feature = "apple")]
    {
        apple::Context::takeover(app)
    }
    #[cfg(feature = "gtk")]
    {
        gtk::Context::takeover(app)
    }

    #[cfg(all(not(feature = "apple"), not(feature = "gtk")))]
    panic!("No platform backend enabled. please enable one of the features: apple, gtk");
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
