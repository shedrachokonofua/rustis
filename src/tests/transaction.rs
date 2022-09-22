use crate::{
    cmd, resp::Value, tests::get_default_addr, Connection, ConnectionCommandResult, Error,
    FlushingMode, ListCommands, PrepareCommand, Result, ServerCommands, StringCommands,
    TransactionCommandResult, TransactionCommands, TransactionExt,
};
use serial_test::serial;

#[cfg_attr(feature = "tokio-runtime", tokio::test)]
#[cfg_attr(feature = "async-std-runtime", async_std::test)]
#[serial]
async fn transaction() -> Result<()> {
    let connection = Connection::connect(get_default_addr()).await?;

    let transaction = connection.create_transaction().await?;

    let value: String = transaction
        .set("key1", "value1")
        .queue_and_forget()
        .await?
        .set("key2", "value2")
        .queue_and_forget()
        .await?
        .get("key1")
        .queue()
        .await?
        .exec()
        .await?;

    assert_eq!("value1", value);

    Ok(())
}

#[cfg_attr(feature = "tokio-runtime", tokio::test)]
#[cfg_attr(feature = "async-std-runtime", async_std::test)]
#[serial]
async fn transaction_error() -> Result<()> {
    let connection = Connection::connect(get_default_addr()).await?;

    let transaction = connection.create_transaction().await?;

    let result = transaction
        .prepare_command::<Value>(cmd("UNKNOWN"))
        .queue()
        .await;
    assert!(
        matches!(result, Err(Error::Redis(e)) if e.starts_with("ERR unknown command 'UNKNOWN'"))
    );

    transaction.discard().await?;

    let transaction = connection.create_transaction().await?;

    let result = transaction
        .set("key1", "abc")
        .queue_and_forget()
        .await?
        .lpop::<_, String, Vec<_>>("key1", 1)
        .queue()
        .await?
        .exec()
        .await;

    assert!(result.is_err());

    Ok(())
}

#[cfg_attr(feature = "tokio-runtime", tokio::test)]
#[cfg_attr(feature = "async-std-runtime", async_std::test)]
#[serial]
async fn watch() -> Result<()> {
    let connection = Connection::connect(get_default_addr()).await?;
    connection.flushdb(FlushingMode::Sync).send().await?;

    connection.set("key", 1).send().await?;
    connection.watch("key").send().await?;

    let mut value: i32 = connection.get("key").send().await?;
    value += 1;

    let transaction = connection.create_transaction().await?;

    transaction.set("key", value).queue().await?.exec().await?;

    let value: i32 = connection.get("key").send().await?;
    assert_eq!(2, value);

    let value = 3;
    connection.watch("key").send().await?;

    let transaction = connection.create_transaction().await?;

    // set key on another connection during the transaction
    let connection2 = Connection::connect(get_default_addr()).await?;
    connection2.set("key", value).send().await?;

    let result = transaction.set("key", value).queue().await?.exec().await;
    assert!(matches!(result, Err(Error::Aborted)));

    Ok(())
}

#[cfg_attr(feature = "tokio-runtime", tokio::test)]
#[cfg_attr(feature = "async-std-runtime", async_std::test)]
#[serial]
async fn unwatch() -> Result<()> {
    let connection = Connection::connect(get_default_addr()).await?;
    connection.flushdb(FlushingMode::Sync).send().await?;

    connection.set("key", 1).send().await?;
    connection.watch("key").send().await?;

    let mut value: i32 = connection.get("key").send().await?;
    value += 1;

    connection.watch("key").send().await?;
    connection.unwatch().send().await?;

    let transaction = connection.create_transaction().await?;

    // set key on another connection during the transaction
    let connection2 = Connection::connect(get_default_addr()).await?;
    connection2.set("key", 3).send().await?;

    transaction.set("key", value).queue().await?.exec().await?;

    let value: i32 = connection.get("key").send().await?;
    assert_eq!(2, value);

    Ok(())
}
