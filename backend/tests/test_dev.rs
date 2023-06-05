use anyhow::Result;

#[tokio::test]
async fn my_test() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;
    hc.do_get("/hello/xxx").await?.print().await?;
    Ok(())
}

#[test]
fn test_add() {
    assert!(true);
}