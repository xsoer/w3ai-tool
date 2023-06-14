mod db;
mod model;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    let db = db::DB::new().await;
    db.read_chart().await;
}
