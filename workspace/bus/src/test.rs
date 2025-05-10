use ctor::ctor;
use std::env::set_var;

use base::config::Config;

#[ctor]
fn setup() {
    unsafe {
        set_var("ENV", "test");
    }
    let _config = Config::load();
}
