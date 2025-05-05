use crate::models::chat::{Message, Group, GroupUser, GroupApply, Friend, ChatFile, FriendStatus};
use surrealdb::Error;
use surrealdb::sql::Thing;
use crate::db::Database;

impl Database {
    // 消息相关方法
    pub async fn create_message(&self, message: &Message) -> Result<(), Error> {
        let sql = "
            CREATE message CONTENT $data;
            UPDATE user:$from_user SET last_active_time = time::now();
        ";
        
        let vars = surrealdb::sql::Object::from([
            ("data".into(), message.clone().into()),
            ("from_user".into(), message.from_user.clone().into()),
        ]);
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.db.query(sql).bind(vars).await?.take(0)?;
        Ok(())
    }
    
    pub async fn get_message_by_id(&self, message_id: &str) -> Result<Option<Message>, Error> {
        let sql = "SELECT * FROM message WHERE msg_id = $msg_id";
        let vars = surrealdb::sql::Object::from([
            ("msg_id".into(), message_id.into()),
        ]);
        
        let mut response = self.db.query(sql).bind(vars).await?;
        let messages: Vec<Message> = response.take(0)?;
        
        Ok(messages.into_iter().next())
    }
    
    pub async fn get_chat_messages(&self, chat_identify: &str, limit: u32, offset: u32) -> Result<Vec<Message>, Error> {
        let sql = "
            SELECT * FROM message 
            WHERE chat_identify = $chat_identify 
            ORDER BY created_at DESC 
            LIMIT $limit 
            START $offset
        ";
        
        let vars = surrealdb::sql::Object::from([
            ("chat_identify".into(), chat_identify.into()),
            ("limit".into(), limit.into()),
            ("offset".into(), offset.into()),
        ]);
        
        let mut response = self.db.query(sql).bind(vars).await?;
        let messages: Vec<Message> = response.take(0)?;
        
        Ok(messages)
    }
    
    pub async fn set_messages_read(&self, chat_identify: &str, user_id: &str) -> Result<(), Error> {
        let sql = "
            UPDATE message 
            SET is_read = true 
            WHERE chat_identify = $chat_identify 
            AND to_user = $user_id 
            AND is_read = false
        ";
        
        let vars = surrealdb::sql::Object::from([
            ("chat_identify".into(), chat_identify.into()),
            ("user_id".into(), user_id.into()),
        ]);
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.db.query(sql).bind(vars).await?.take(0)?;
        Ok(())
    }
    
    pub async fn delete_message(&self, message_id: &str) -> Result<(), Error> {
        let sql = "DELETE message WHERE msg_id = $msg_id";
        let vars = surrealdb::sql::Object::from([
            ("msg_id".into(), message_id.into()),
        ]);
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.db.query(sql).bind(vars).await?.take(0)?;
        Ok(())
    }
    
    // 群组相关方法
    pub async fn create_group(&self, group: &Group) -> Result<(), Error> {
        let sql = "CREATE group CONTENT $data";
        let vars = surrealdb::sql::Object::from([
            ("data".into(), group.clone().into()),
        ]);
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.db.query(sql).bind(vars).await?.take(0)?;
        Ok(())
    }
    
    pub async fn get_group_by_id(&self, group_id: &str) -> Result<Option<Group>, Error> {
        let sql = "SELECT * FROM group WHERE group_id = $group_id";
        let vars = surrealdb::sql::Object::from([
            ("group_id".into(), group_id.into()),
        ]);
        
        let mut response = self.db.query(sql).bind(vars).await?;
        let groups: Vec<Group> = response.take(0)?;
        
        Ok(groups.into_iter().next())
    }
    
    pub async fn update_group(&self, group: &Group) -> Result<(), Error> {
        let sql = "UPDATE group SET $data WHERE group_id = $group_id";
        let vars = surrealdb::sql::Object::from([
            ("data".into(), group.clone().into()),
            ("group_id".into(), group.group_id.clone().into()),
        ]);
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.db.query(sql).bind(vars).await?.take(0)?;
        Ok(())
    }
    
    pub async fn delete_group(&self, group_id: &str) -> Result<(), Error> {
        let sql = "DELETE group WHERE group_id = $group_id";
        let vars = surrealdb::sql::Object::from([
            ("group_id".into(), group_id.into()),
        ]);
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.db.query(sql).bind(vars).await?.take(0)?;
        Ok(())
    }
    
    // 群组成员相关方法
    pub async fn create_group_user(&self, group_user: &GroupUser) -> Result<(), Error> {
        let sql = "CREATE group_user CONTENT $data";
        let vars = surrealdb::sql::Object::from([
            ("data".into(), group_user.clone().into()),
        ]);
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.db.query(sql).bind(vars).await?.take(0)?;
        Ok(())
    }
    
    pub async fn get_group_users(&self, group_id: &str) -> Result<Vec<GroupUser>, Error> {
        let sql = "SELECT * FROM group_user WHERE group_id = $group_id";
        let vars = surrealdb::sql::Object::from([
            ("group_id".into(), group_id.into()),
        ]);
        
        let mut response = self.db.query(sql).bind(vars).await?;
        let group_users: Vec<GroupUser> = response.take(0)?;
        
        Ok(group_users)
    }
    
    pub async fn get_user_groups(&self, user_id: &str) -> Result<Vec<GroupUser>, Error> {
        let sql = "SELECT * FROM group_user WHERE user_id = $user_id";
        let vars = surrealdb::sql::Object::from([
            ("user_id".into(), user_id.into()),
        ]);
        
        let mut response = self.db.query(sql).bind(vars).await?;
        let group_users: Vec<GroupUser> = response.take(0)?;
        
        Ok(group_users)
    }
    
    pub async fn delete_group_user(&self, group_id: &str, user_id: &str) -> Result<(), Error> {
        let sql = "DELETE group_user WHERE group_id = $group_id AND user_id = $user_id";
        let vars = surrealdb::sql::Object::from([
            ("group_id".into(), group_id.into()),
            ("user_id".into(), user_id.into()),
        ]);
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.db.query(sql).bind(vars).await?.take(0)?;
        Ok(())
    }
    
    // 群组申请相关方法
    pub async fn create_group_apply(&self, apply: &GroupApply) -> Result<(), Error> {
        let sql = "CREATE group_apply CONTENT $data";
        let vars = surrealdb::sql::Object::from([
            ("data".into(), apply.clone().into()),
        ]);
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.db.query(sql).bind(vars).await?.take(0)?;
        Ok(())
    }
    
    pub async fn get_group_applies(&self, group_id: &str) -> Result<Vec<GroupApply>, Error> {
        let sql = "SELECT * FROM group_apply WHERE group_id = $group_id AND status = 0";
        let vars = surrealdb::sql::Object::from([
            ("group_id".into(), group_id.into()),
        ]);
        
        let mut response = self.db.query(sql).bind(vars).await?;
        let applies: Vec<GroupApply> = response.take(0)?;
        
        Ok(applies)
    }
    
    pub async fn update_group_apply(&self, apply_id: &str, status: i32) -> Result<(), Error> {
        let sql = "UPDATE group_apply SET status = $status WHERE id = $apply_id";
        let vars = surrealdb::sql::Object::from([
            ("apply_id".into(), apply_id.into()),
            ("status".into(), status.into()),
        ]);
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.db.query(sql).bind(vars).await?.take(0)?;
        Ok(())
    }
    
    // 好友相关方法
    pub async fn create_friend(&self, friend: &Friend) -> Result<(), Error> {
        let sql = "CREATE friend CONTENT $data";
        let vars = surrealdb::sql::Object::from([
            ("data".into(), friend.clone().into()),
        ]);
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.db.query(sql).bind(vars).await?.take(0)?;
        Ok(())
    }
    
    pub async fn get_friends(&self, user_id: &str) -> Result<Vec<Friend>, Error> {
        let sql = "
            SELECT * FROM friend 
            WHERE user_id = $user_id 
            AND status = $normal_status
        ";
        let vars = surrealdb::sql::Object::from([
            ("user_id".into(), user_id.into()),
            ("normal_status".into(), (FriendStatus::Normal as i32).into()),
        ]);
        
        let mut response = self.db.query(sql).bind(vars).await?;
        let friends: Vec<Friend> = response.take(0)?;
        
        Ok(friends)
    }
    
    pub async fn get_friend_applies(&self, user_id: &str) -> Result<Vec<Friend>, Error> {
        let sql = "
            SELECT * FROM friend 
            WHERE friend_id = $user_id 
            AND status = $applying_status
        ";
        let vars = surrealdb::sql::Object::from([
            ("user_id".into(), user_id.into()),
            ("applying_status".into(), (FriendStatus::Applying as i32).into()),
        ]);
        
        let mut response = self.db.query(sql).bind(vars).await?;
        let applies: Vec<Friend> = response.take(0)?;
        
        Ok(applies)
    }
    
    pub async fn get_friend_by_id(&self, friend_id: &str) -> Result<Option<Friend>, Error> {
        let sql = "SELECT * FROM friend WHERE id = $friend_id";
        let vars = surrealdb::sql::Object::from([
            ("friend_id".into(), friend_id.into()),
        ]);
        
        let mut response = self.db.query(sql).bind(vars).await?;
        let friends: Vec<Friend> = response.take(0)?;
        
        Ok(friends.into_iter().next())
    }
    
    pub async fn get_friend(&self, user_id: &str, friend_id: &str) -> Result<Option<Friend>, Error> {
        let sql = "
            SELECT * FROM friend 
            WHERE user_id = $user_id 
            AND friend_id = $friend_id
        ";
        let vars = surrealdb::sql::Object::from([
            ("user_id".into(), user_id.into()),
            ("friend_id".into(), friend_id.into()),
        ]);
        
        let mut response = self.db.query(sql).bind(vars).await?;
        let friends: Vec<Friend> = response.take(0)?;
        
        Ok(friends.into_iter().next())
    }
    
    pub async fn update_friend_status(&self, friend_id: &str, status: i32) -> Result<(), Error> {
        let sql = "UPDATE friend SET status = $status WHERE id = $friend_id";
        let vars = surrealdb::sql::Object::from([
            ("friend_id".into(), friend_id.into()),
            ("status".into(), status.into()),
        ]);
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.db.query(sql).bind(vars).await?.take(0)?;
        Ok(())
    }
    
    pub async fn update_friend_remark(&self, friend_id: &str, remark: Option<String>) -> Result<(), Error> {
        let sql = "UPDATE friend SET remark = $remark WHERE id = $friend_id";
        let vars = surrealdb::sql::Object::from([
            ("friend_id".into(), friend_id.into()),
            ("remark".into(), remark.into()),
        ]);
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.db.query(sql).bind(vars).await?.take(0)?;
        Ok(())
    }
    
    pub async fn delete_friend(&self, friend_id: &str) -> Result<(), Error> {
        let sql = "DELETE friend WHERE id = $friend_id";
        let vars = surrealdb::sql::Object::from([
            ("friend_id".into(), friend_id.into()),
        ]);
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.db.query(sql).bind(vars).await?.take(0)?;
        Ok(())
    }
    
    // 群组用户角色更新
    pub async fn update_group_user_role(&self, group_id: &str, user_id: &str, role: i32) -> Result<(), Error> {
        let sql = "
            UPDATE group_user 
            SET role = $role 
            WHERE group_id = $group_id 
            AND user_id = $user_id
        ";
        let vars = surrealdb::sql::Object::from([
            ("group_id".into(), group_id.into()),
            ("user_id".into(), user_id.into()),
            ("role".into(), role.into()),
        ]);
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.db.query(sql).bind(vars).await?.take(0)?;
        Ok(())
    }
    
    // 获取群组申请信息
    pub async fn get_group_apply_by_id(&self, apply_id: &str) -> Result<Option<GroupApply>, Error> {
        let sql = "SELECT * FROM group_apply WHERE id = $apply_id";
        let vars = surrealdb::sql::Object::from([
            ("apply_id".into(), apply_id.into()),
        ]);
        
        let mut response = self.db.query(sql).bind(vars).await?;
        let applies: Vec<GroupApply> = response.take(0)?;
        
        Ok(applies.into_iter().next())
    }
    
    // 文件相关方法
    pub async fn create_file(&self, file: &ChatFile) -> Result<(), Error> {
        let sql = "CREATE chat_file CONTENT $data";
        let vars = surrealdb::sql::Object::from([
            ("data".into(), file.clone().into()),
        ]);
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.db.query(sql).bind(vars).await?.take(0)?;
        Ok(())
    }
    
    pub async fn get_file_by_id(&self, file_id: &str) -> Result<Option<ChatFile>, Error> {
        let sql = "SELECT * FROM chat_file WHERE id = $file_id";
        let vars = surrealdb::sql::Object::from([
            ("file_id".into(), file_id.into()),
        ]);
        
        let mut response = self.db.query(sql).bind(vars).await?;
        let files: Vec<ChatFile> = response.take(0)?;
        
        Ok(files.into_iter().next())
    }
}
