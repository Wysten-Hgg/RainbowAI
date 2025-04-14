use crate::db::Database;

pub async fn reset_daily_limits(db: &Database) -> Result<(), Box<dyn std::error::Error>> {
    // 获取所有用户
    let users = db.get_all_users().await?;
    
    // 重置每个用户的每日限制
    for mut user in users {
        user.daily_chat_count = 0;
        user.daily_lio_count = 0;
        db.update_user(&user).await?;
    }

    Ok(())
}
