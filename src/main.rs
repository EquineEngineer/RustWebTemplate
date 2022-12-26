#![allow(dead_code)]

use std::{env, sync::Arc};

use tracing_subscriber::{
    filter::LevelFilter, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
};

use dotenv::dotenv;

mod prisma;

use rspc::Router;

struct PrismaDbCtx {
    db: Arc<prisma::PrismaClient>,
}

fn router() -> Router<PrismaDbCtx> {
    Router::<PrismaDbCtx>::new()
        .query("version", |t| t(|_ctx, _: ()| "0.0.1"))
        .build()
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!("App starting");

    tracing_subscriber::registry()
        .with(LevelFilter::TRACE)
        .with(tracing_subscriber::fmt::layer())
        .init();

    println!("App creating connection");

    let _router = router();

    _router
        .export_ts("src/generated.ts")
        .expect("Error exporting typescript definitions");

    let client = prisma::new_client_with_url(
        &env::var("DATABASE_URL").expect("`DATABASE_URL` not provided"),
    )
    .await
    .expect("Error creating DB client");

    client.post().delete_many(vec![]).exec().await.unwrap();

    client
        .post()
        .create("My Post".into(), "Hello World".into(), vec![])
        .exec()
        .await
        .unwrap();

    println!("App finished successfully");
}
