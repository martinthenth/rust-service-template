use ctor::ctor;

use crate::config::Config;

#[ctor]
fn setup() {
    let _config = Config::load();
}
