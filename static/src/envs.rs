use std::env;

use tracing::info;

pub async fn load_envs() {
    info!("ENV Precheck");
    let envs: Vec<&str> = vec!["DATABASE_URL", "PARENT_IMAGE_DIR"];
    for env in envs {
        let env_val = env::var(env);
        if env_val.is_err() {
            panic!("{} nincs setelve!", env);
        }
    }
}
