use crate::{
    tests::get_default_addr, GenericCommands,
    HashCommands, Result, SetCommands, Connection, ConnectionCommandResult,
};
use serial_test::serial;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

#[cfg_attr(feature = "tokio-runtime", tokio::test)]
#[cfg_attr(feature = "async-std-runtime", async_std::test)]
#[serial]
async fn key_value_collection() -> Result<()> {
    let connection = Connection::connect(get_default_addr()).await?;

    connection.del("key").send().await?;
    let items = ("field1", "value1");
    let len = connection.hset("key", items).send().await?;
    assert_eq!(1, len);

    connection.del("key").send().await?;
    let items = HashMap::from([("field1", "value1"), ("field2", "value2")]);
    let len = connection.hset("key", items).send().await?;
    assert_eq!(2, len);

    connection.del("key").send().await?;
    let items = BTreeMap::from([("field1", "value1"), ("field2", "value2")]);
    let len = connection.hset("key", items).send().await?;
    assert_eq!(2, len);

    connection.del("key").send().await?;
    let items = vec![("field1", "value1"), ("field2", "value2")];
    let len = connection.hset("key", items).send().await?;
    assert_eq!(2, len);

    connection.del("key").send().await?;
    let items = [("field1", "value1"), ("field2", "value2")];
    let len = connection.hset("key", items).send().await?;
    assert_eq!(2, len);

    Ok(())
}

#[cfg_attr(feature = "tokio-runtime", tokio::test)]
#[cfg_attr(feature = "async-std-runtime", async_std::test)]
#[serial]
async fn set_collection() -> Result<()> {
    let connection = Connection::connect(get_default_addr()).await?;

    connection.del("key").send().await?;
    let items = "member1";
    let len = connection.sadd("key", items).send().await?;
    assert_eq!(1, len);

    connection.del("key").send().await?;
    let items = ["member1", "member2"];
    let len = connection.sadd("key", items).send().await?;
    assert_eq!(2, len);

    connection.del("key").send().await?;
    let items = vec!["member1", "member2"];
    let len = connection.sadd("key", items).send().await?;
    assert_eq!(2, len);

    connection.del("key").send().await?;
    let items = HashSet::from(["member1", "member2"]);
    let len = connection.sadd("key", items).send().await?;
    assert_eq!(2, len);

    connection.del("key").send().await?;
    let items = BTreeSet::from(["member1", "member2"]);
    let len = connection.sadd("key", items).send().await?;
    assert_eq!(2, len);

    Ok(())
}
