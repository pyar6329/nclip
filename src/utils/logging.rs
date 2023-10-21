use std::convert::AsRef;
use tracing::{error, info, warn};

pub fn info_msg<S>(s: S) -> String
where
    S: AsRef<str>,
{
    let msg = s.as_ref();
    info!("{}", msg);
    msg.to_string()
}

pub fn warn_msg<S>(s: S) -> String
where
    S: AsRef<str>,
{
    let msg = s.as_ref();
    warn!("{}", msg);
    msg.to_string()
}

pub fn err_msg<S>(s: S) -> String
where
    S: AsRef<str>,
{
    let msg = s.as_ref();
    error!("{}", msg);
    msg.to_string()
}
