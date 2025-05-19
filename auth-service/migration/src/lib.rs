pub use sea_orm_migration::prelude::*;

mod m20250420_030008_users;
mod m20250420_030033_access_tokens;
mod m20250502_130004_roles;
mod m20250502_131410_permissions;
mod m20250504_221124_role_permissions;
mod m20250505_061040_user_roles;
mod m20250505_061041_seed_role_permission;
mod m20250505_161056_seed_create_admin;
mod m20250519_022040_attempt_logins;
mod m20250519_022053_failed_logins;
mod m20250519_023508_banned_logins;
mod m20250519_142643_system_configs;
mod m20250519_142758_seed_create_system_config;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250420_030008_users::Migration),
            Box::new(m20250420_030033_access_tokens::Migration),
            Box::new(m20250502_130004_roles::Migration),
            Box::new(m20250502_131410_permissions::Migration),
            Box::new(m20250504_221124_role_permissions::Migration),
            Box::new(m20250505_061040_user_roles::Migration),
            Box::new(m20250505_061041_seed_role_permission::Migration),
            Box::new(m20250505_161056_seed_create_admin::Migration),
            Box::new(m20250519_022040_attempt_logins::Migration),
            Box::new(m20250519_022053_failed_logins::Migration),
            Box::new(m20250519_023508_banned_logins::Migration),
            Box::new(m20250519_142643_system_configs::Migration),
            Box::new(m20250519_142758_seed_create_system_config::Migration),
        ]
    }
}
