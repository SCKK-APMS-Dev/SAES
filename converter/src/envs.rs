use std::env;

pub async fn load_envs() {
    println!("ENV Precheck");
    let envs: Vec<&str> = vec!["DATABASE_URL", "CONVERT_DIR"];
    for env in envs {
        let env_val = env::var(env);
        if env_val.is_err() {
            panic!("{} nincs setelve!", env);
        }
    }
}
