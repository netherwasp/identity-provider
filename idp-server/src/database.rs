use sqlx::{Executor, PgPool, postgres::PgPoolOptions};
use url::Url;

#[derive(Debug, Clone)]
pub struct IdentityDatabase {
    pub super_admin_url: String,
    pub idp_admin_url: Option<String>,
}

impl IdentityDatabase {
    pub async fn set_super_admin_url(&self, admin_url: String) -> Self {
        Self {
            super_admin_url: admin_url,
            idp_admin_url: None,
        }
    }

    pub async fn super_admin_connect(&self) -> Result<PgPool, String> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(self.super_admin_url.as_str())
            .await
            .map_err(|e| format!("Error: {:?}", e))?;
        Ok(pool)
    }

    pub async fn idp_admin_connect(&self) -> Result<PgPool, String> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(self.idp_admin_url.clone().unwrap_or_default().as_str())
            .await
            .map_err(|e| format!("Error: {:?}", e))?;
        Ok(pool)
    }

    // pub async fn idp_admin_create(
    //     &mut self,
    //     username: String,
    //     password: String,
    // ) -> Result<Self, String> {
    //     let super_admin_pool = self.super_admin_connect().await.unwrap();

    //     Ok(Self {
    //         super_admin_url: self.super_admin_url.clone(),
    //         idp_admin_url: self.idp_admin_url.clone(),
    //     })
    // }

    pub async fn idp_db_init(&mut self) -> Result<Self, String> {
        let super_admin_pool = self.super_admin_connect().await.unwrap();
        let url = Url::parse(self.idp_admin_url.clone().unwrap().as_str()).expect("Invalid Url");
        let username = url.username().to_string();
        let password = url.password().unwrap_or_default().to_string();
        let db_name = url.path().trim_start_matches('/').to_string();

        if !sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = $1)",
        )
        .bind(username.clone())
        .fetch_one(&super_admin_pool)
        .await
        .unwrap()
        {
            super_admin_pool
                .execute(
                    format!(
                        "CREATE USER {username} WITH PASSWORD '{password}';
                        ALTER USER {username} WITH CREATEDB;"
                    )
                    .as_str(),
                )
                .await
                .map_err(|e| format!("Error: {:?}", e))?;
        };

        if !sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS (SELECT 1 FROM pg_database WHERE datname = $1)",
        )
        .bind(db_name.clone())
        .fetch_one(&super_admin_pool)
        .await
        .unwrap()
        {
            super_admin_pool
                .execute(format!("CREATE DATABASE {} OWNER {};", db_name, username).as_str())
                .await
                .map_err(|e| format!("Error: {:?}", e))?;
        }

        Ok(Self {
            super_admin_url: self.super_admin_url.clone(),
            idp_admin_url: self.idp_admin_url.clone(),
        })
    }
}
