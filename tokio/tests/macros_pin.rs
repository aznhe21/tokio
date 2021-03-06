use tokio::pin;

async fn one() {}
async fn two() {}

#[tokio::test]
async fn multi_pin() {
    pin! {
        let f1 = one();
        let f2 = two();
    }

    (&mut f1).await;
    (&mut f2).await;
}
