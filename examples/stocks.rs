use std::time::Instant;

use sherpa::stock_streamer_client::StockStreamerClient;
use stock::StockRequest;
use tonic::Request;

pub mod sherpa {
    tonic::include_proto!("sherpa");
}

pub mod stock {
    tonic::include_proto!("stock");
}
pub mod crypto {
    tonic::include_proto!("crypto");
}
pub mod news {
    tonic::include_proto!("news");
}

async fn connect_and_stream() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("starting gRPC client");

    let mut client = StockStreamerClient::connect("http://localhost:10000").await?;
    let mut stream = client
        .get_stock(Request::new(StockRequest {
            trade: true,
            trade_correction: true,
            trade_cancel: true,
            quote: true,
            bar: true,
            daily_bar: true,
            updated_bar: true,
            luld: true,
            status: true,
        }))
        .await?
        .into_inner();

    let mut counter = 0;
    let start_time = Instant::now();
    while let Some(tick) = stream.message().await? {
        counter += 1;
        println!("{:?}", tick);
        _ = tick;
        // if counter % 10_000 == 0 {
        //     println!("   got {counter}");
        // }
    }

    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!(
        "final count is {counter} {:?}, {:?}",
        elapsed_time,
        counter as f64 / elapsed_time.as_secs_f64()
    );
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let thread_count = 1;
    let mut handles = vec![];
    for _ in 0..thread_count {
        let h = tokio::spawn(connect_and_stream());
        handles.push(h)
    }

    for handle in handles {
        let _ = handle.await.expect("blah");
    }
    Ok(())
}
