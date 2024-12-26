use serde::{ Deserialize, Serialize };

#[derive(Deserialize)]
pub struct MusicLog {
    pub name: String,
    pub directory: String,
    pub time: String,
    pub user_directory: String,
}

pub async fn add_logs(client: &mut tokio_postgres::Client, logs: Vec<MusicLog>)-> Result<(), Box<dyn std::error::Error>> {

    let trans = client.transaction().await?;

    for item in logs {
        let MusicLog { name, directory, user_directory, time: create_time } = item;
        let rows = trans.query(
            "SELECT * FROM music_info WHERE user_directory = $1 AND name = $2",
            &[
                &user_directory,
                &name,
            ]
        ).await?;
        // 如果没有，就添加一下
        if rows.len() == 0 {
            trans.query(
                "INSERT INTO music_info (user_directory, name, directory) VALUES ($1, $2, $3)",
                &[
                    &user_directory,
                    &name,
                    &directory,
                ],
            ).await?;
        }
        trans.query(
            &format!("INSERT INTO listen_logs (user_directory, name, create_time) VALUES ($1, $2, '{}')", &create_time),
            &[
                &user_directory,
                &name,
            ],
        ).await?;
    }
    trans.commit().await?;

    println!("数据添加完成");

    Ok(())
}