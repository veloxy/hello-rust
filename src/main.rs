mod models;
mod routes;

use sea_orm::{Database};
use migration::{Migrator, MigratorTrait};

const DATABASE_URL: &str = "sqlite:./var/sqlite.db?mode=rwc";

#[tokio::main]
async fn main() {
    let connection = Database::connect(DATABASE_URL).await.unwrap();
    Migrator::up(&connection, None).await.unwrap();

    let router = routes::create_routes(connection);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap()
}
