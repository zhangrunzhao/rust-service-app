// region:    --- Modules
mod error;

pub use self::error::{Error, Result};

// endregion: --- Modules

// region:    --- Constructor
#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: i64,
}

impl Ctx {
    pub fn root_ctx() -> Self {
        Ctx { user_id: 0 }
    }

    pub fn new(user_id: i64) -> Result<Self> {
        if user_id == 0 {
            Err(Error::CtxCannotNewRootCtx)
        } else {
            Ok(Self { user_id })
        }
    }
}

// endregion: --- Constructor

// region:    --- Property Accessors

impl Ctx {
    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}

// endregion: --- Property Accessors
