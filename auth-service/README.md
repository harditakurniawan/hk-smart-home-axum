# HK Smart Home Axum - Authentication Service

## How to run application :
1. Create .env file. Copas the value from .env.example
2. Create jwt-private.key & jwt-public.key in folder src/config
3. Run migration: sea-orm-cli migrate refresh
4. Generate or update entity: sea-orm-cli generate entity -o entity/src
5. Install dependency: make install
6. Run app: make run-watch or make run (for production)