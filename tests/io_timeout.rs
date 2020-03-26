use std::time::Duration;

use async_std::io;
use async_std::task;
/* timeout behaviur is different in tokio. It doesn't drop the future.
#[tokio::test]
#[should_panic(expected = "future timed out")]
async fn io_timeout_timedout() {
        io::timeout(Duration::from_secs(1), async {

            let stdin = io::stdin();
            let mut line = String::new();

            let _n = stdin.read_line(&mut line).await.unwrap();

            Ok(())
        })
        .await
        .unwrap(); // We should panic with a timeout error

}
*/
#[test]
#[should_panic(expected = "My custom error")]
fn io_timeout_future_err() {
    task::block_on(async {
        io::timeout(Duration::from_secs(1), async {
            Err::<(), io::Error>(io::Error::new(io::ErrorKind::Other, "My custom error"))
        })
        .await
        .unwrap(); // We should panic with our own error
    });
}

#[test]
fn io_timeout_future_ok() {
    task::block_on(async {
        io::timeout(Duration::from_secs(1), async { Ok(()) })
            .await
            .unwrap(); // We shouldn't panic at all
    });
}
