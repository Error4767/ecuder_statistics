pub async fn create_table(client: &mut tokio_postgres::Client)-> Result<(), Box<dyn std::error::Error>> {
  let trans = client.transaction().await?;
  let sql_content = tokio::fs::read_to_string("./sql/cloud_music_statistics/listen_logs.sql").await?;
  trans.query(&sql_content, &[]).await?;
  let sql_content = tokio::fs::read_to_string("./sql/cloud_music_statistics/music_info.sql").await?;
  trans.query(&sql_content, &[]).await?;
  trans.commit().await?;
  Ok(())
}

