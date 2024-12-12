pub mod ad_operations;

use ldap3::{result::Result as LdapResult, Ldap, LdapConnAsync};
use std::env;

#[derive(Debug, Clone)]
pub struct LdapConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub base_dn: String,
}

impl LdapConfig {
    pub fn load() -> Result<Self, env::VarError> {
        dotenv::dotenv().ok();

        let host = env::var("LDAP_HOST")?;
        let port: u16 = env::var("LDAP_PORT")
            .unwrap_or_else(|_| "389".to_string())
            .parse()
            .expect("LDAP_PORT must be a number");
        let user = env::var("LDAP_USER")?;
        let password = env::var("LDAP_PASSWORD")?;
        let base_dn = env::var("LDAP_BASE_DN")?;

        Ok(LdapConfig {
            host,
            port,
            user,
            password,
            base_dn,
        })
    }
}

pub async fn connect_to_ldap(config: &LdapConfig) -> LdapResult<Ldap> {
    let addr = format!("ldap://{}:{}", config.host, config.port);
    let (conn, mut ldap) = LdapConnAsync::new(&addr).await?;
    ldap3::drive!(conn);
    ldap.simple_bind(&config.user, &config.password).await?.success()?;
    Ok(ldap)
}