use sea_orm_migration::{prelude::*, sea_orm::{ActiveValue::Set, EntityTrait, PaginatorTrait}};
use entity::{permissions, role_permissions, roles};

#[derive(DeriveMigrationName)]
pub struct Migration;

pub const ADMIN_ROLE: &str = "admin";
pub const USER_ROLE: &str = "user";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db: &SchemaManagerConnection<'_> = manager.get_connection();

        let is_roles_exists = roles::Entity::find().count(db).await?;

        if is_roles_exists > 0 {
            println!("seed_role_permission already generated");
            return Ok(());
        }

        async_std::task::block_on(async {
            create_roles(db).await;
            create_permissions(db).await;
        });

        create_role_permission(db).await;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

async fn create_roles(db: &SchemaManagerConnection<'_>) {
    let roles: Vec<roles::ActiveModel> = vec![
            roles::ActiveModel {
                name: Set(ADMIN_ROLE.to_string()),
                ..Default::default()
            },
            roles::ActiveModel {
                name: Set(USER_ROLE.to_string()),
                ..Default::default()
            },
        ];

        if let Err(err) = roles::Entity::insert_many(roles).exec(db).await {
            eprintln!("Failed to insert seed_role_permission | roles: {:?}", err);
        }
}

async fn create_permissions(db: &SchemaManagerConnection<'_>) {
    let permissions = vec![
        permissions::ActiveModel {
            name: Set("read statistic".to_string().to_uppercase()),
            prefix: Set("read_statistic".to_string()),
            ..Default::default()
        },
        permissions::ActiveModel {
            name: Set("create device".to_string().to_uppercase()),
            prefix: Set("create_device".to_string()),
            ..Default::default()
        },
        permissions::ActiveModel {
            name: Set("read device".to_string().to_uppercase()),
            prefix: Set("read_device".to_string()),
            ..Default::default()
        },
        permissions::ActiveModel {
            name: Set("update device".to_string().to_uppercase()),
            prefix: Set("read_device".to_string()),
            ..Default::default()
        },
        permissions::ActiveModel {
            name: Set("delete device".to_string().to_uppercase()),
            prefix: Set("read_device".to_string()),
            ..Default::default()
        },
    ];

    if let Err(err) = permissions::Entity::insert_many(permissions).exec(db).await {
        eprintln!("Failed to insert seed_role_permission | permissions: {:?}", err);
    }
}

async fn create_role_permission(db: &SchemaManagerConnection<'_>) {
    let roles = roles::Entity::find().all(db).await;
    let permissions = permissions::Entity::find().all(db).await;
    
    if let (Ok(roles), Ok(permissions)) = (roles, permissions) {
        let admin_role = roles.iter().find(|x| x.name == ADMIN_ROLE).unwrap();
        let user_role = roles.iter().find(|x| x.name == USER_ROLE).unwrap();

        let admin_permission = permissions.iter().find(|x| x.prefix == "read_statistic").unwrap();
        let user_permissions = permissions.iter().filter(|x| x.prefix != "read_statistic");

        let admin_role_permission = role_permissions::ActiveModel {
            role_id: Set(admin_role.id.clone()),
            permission_id: Set(admin_permission.id.clone()),
            ..Default::default()
        };

        if let Err(err) = role_permissions::Entity::insert(admin_role_permission).exec(db).await {
            eprintln!("Failed to insert seed_role_permission | admin role permission: {:?}", err);
        }

        for user_permission in user_permissions {
            let user_role_permission = role_permissions::ActiveModel {
                role_id: Set(user_role.id.clone()),
                permission_id: Set(user_permission.id.clone()),
                ..Default::default()
            };

            if let Err(err) = role_permissions::Entity::insert(user_role_permission).exec(db).await {
                eprintln!("Failed to insert seed_role_permission | user role permission: {:?}", err);
            }
        }
    }
}