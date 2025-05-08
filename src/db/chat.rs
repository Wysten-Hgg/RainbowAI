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
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.client
            .query(sql)
            .bind(("data", message.clone()))
            .bind(("from_user", message.from_user.clone()))
            .await?
            .take(0)?;
        Ok(())
    }
    
    pub async fn get_message_by_id(&self, message_id: &str) -> Result<Option<Message>, Error> {
        let sql = "SELECT * FROM message WHERE msg_id = $msg_id";
        
        let mut response = self.client
            .query(sql)
            .bind(("msg_id", message_id))
            .await?;
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
        
        let mut response = self.client
            .query(sql)
            .bind(("chat_identify", chat_identify))
            .bind(("limit", limit))
            .bind(("offset", offset))
            .await?;
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
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.client
            .query(sql)
            .bind(("chat_identify", chat_identify))
            .bind(("user_id", user_id))
            .await?
            .take(0)?;
        Ok(())
    }
    
    pub async fn delete_message(&self, message_id: &str) -> Result<(), Error> {
        let sql = "DELETE message WHERE msg_id = $msg_id";
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.client
            .query(sql)
            .bind(("msg_id", message_id))
            .await?
            .take(0)?;
        Ok(())
    }
    
    // 群组相关方法
    pub async fn create_group(&self, group: &Group) -> Result<(), Error> {
        let sql = "CREATE group CONTENT $data";
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.client
            .query(sql)
            .bind(("data", group.clone()))
            .await?
            .take(0)?;
        Ok(())
    }
    
    pub async fn get_group_by_id(&self, group_id: &str) -> Result<Option<Group>, Error> {
        let sql = "SELECT * FROM group WHERE group_id = $group_id";
        
        let mut response = self.client
            .query(sql)
            .bind(("group_id", group_id))
            .await?;
        let groups: Vec<Group> = response.take(0)?;
        
        Ok(groups.into_iter().next())
    }
    
    pub async fn update_group(&self, group: &Group) -> Result<(), Error> {
        let sql = "UPDATE group SET $data WHERE group_id = $group_id";
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.client
            .query(sql)
            .bind(("data", group.clone()))
            .bind(("group_id", group.group_id.clone()))
            .await?
            .take(0)?;
        Ok(())
    }
    
    pub async fn delete_group(&self, group_id: &str) -> Result<(), Error> {
        let sql = "DELETE group WHERE group_id = $group_id";
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.client
            .query(sql)
            .bind(("group_id", group_id))
            .await?
            .take(0)?;
        Ok(())
    }
    
    // 群组成员相关方法
    pub async fn create_group_user(&self, group_user: &GroupUser) -> Result<(), Error> {
        let sql = "CREATE group_user CONTENT $data";
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.client
            .query(sql)
            .bind(("data", group_user.clone()))
            .await?
            .take(0)?;
        Ok(())
    }
    
    pub async fn get_group_users(&self, group_id: &str) -> Result<Vec<GroupUser>, Error> {
        let sql = "SELECT * FROM group_user WHERE group_id = $group_id";
        
        let mut response = self.client
            .query(sql)
            .bind(("group_id", group_id))
            .await?;
        let group_users: Vec<GroupUser> = response.take(0)?;
        
        Ok(group_users)
    }
    
    pub async fn get_user_groups(&self, user_id: &str) -> Result<Vec<GroupUser>, Error> {
        let sql = "SELECT * FROM group_user WHERE user_id = $user_id";
        
        let mut response = self.client
            .query(sql)
            .bind(("user_id", user_id))
            .await?;
        let group_users: Vec<GroupUser> = response.take(0)?;
        
        Ok(group_users)
    }
    
    pub async fn delete_group_user(&self, group_id: &str, user_id: &str) -> Result<(), Error> {
        let sql = "DELETE group_user WHERE group_id = $group_id AND user_id = $user_id";
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.client
            .query(sql)
            .bind(("group_id", group_id))
            .bind(("user_id", user_id))
            .await?
            .take(0)?;
        Ok(())
    }
    
    // 群组申请相关方法
    pub async fn create_group_apply(&self, apply: &GroupApply) -> Result<(), Error> {
        let sql = "CREATE group_apply CONTENT $data";
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.client
            .query(sql)
            .bind(("data", apply.clone()))
            .await?
            .take(0)?;
        Ok(())
    }
    
    pub async fn get_group_applies(&self, group_id: &str) -> Result<Vec<GroupApply>, Error> {
        let sql = "SELECT * FROM group_apply WHERE group_id = $group_id AND status = 0";
        
        let mut response = self.client
            .query(sql)
            .bind(("group_id", group_id))
            .await?;
        let applies: Vec<GroupApply> = response.take(0)?;
        
        Ok(applies)
    }
    
    pub async fn update_group_apply(&self, apply_id: &str, status: i32) -> Result<(), Error> {
        let sql = "UPDATE group_apply SET status = $status WHERE id = $apply_id";
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.client
            .query(sql)
            .bind(("apply_id", apply_id))
            .bind(("status", status))
            .await?
            .take(0)?;
        Ok(())
    }
    
    // 好友相关方法
    pub async fn create_friend(&self, friend: &Friend) -> Result<(), Error> {
        let sql = "CREATE friend CONTENT $data";
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.client
            .query(sql)
            .bind(("data", friend.clone()))
            .await?
            .take(0)?;
        Ok(())
    }
    
    pub async fn get_friends(&self, user_id: &str) -> Result<Vec<Friend>, Error> {
        let sql = "
            SELECT * FROM friend 
            WHERE user_id = $user_id 
            AND status = $normal_status
        ";
        
        let mut response = self.client
            .query(sql)
            .bind(("user_id", user_id))
            .bind(("normal_status", FriendStatus::Normal as i32))
            .await?;
        let friends: Vec<Friend> = response.take(0)?;
        
        Ok(friends)
    }
    
    pub async fn get_friend_applies(&self, user_id: &str) -> Result<Vec<Friend>, Error> {
        let sql = "
            SELECT * FROM friend 
            WHERE friend_id = $user_id 
            AND status = $applying_status
        ";
        
        let mut response = self.client
            .query(sql)
            .bind(("user_id", user_id))
            .bind(("applying_status", FriendStatus::Applying as i32))
            .await?;
        let applies: Vec<Friend> = response.take(0)?;
        
        Ok(applies)
    }
    
    pub async fn get_friend_by_id(&self, friend_id: &str) -> Result<Option<Friend>, Error> {
        let sql = "SELECT * FROM friend WHERE id = $friend_id";
        
        let mut response = self.client
            .query(sql)
            .bind(("friend_id", friend_id))
            .await?;
        let friends: Vec<Friend> = response.take(0)?;
        
        Ok(friends.into_iter().next())
    }
    
    pub async fn get_friend(&self, user_id: &str, friend_id: &str) -> Result<Option<Friend>, Error> {
        let sql = "
            SELECT * FROM friend 
            WHERE user_id = $user_id 
            AND friend_id = $friend_id
        ";
        
        let mut response = self.client
            .query(sql)
            .bind(("user_id", user_id))
            .bind(("friend_id", friend_id))
            .await?;
        let friends: Vec<Friend> = response.take(0)?;
        
        Ok(friends.into_iter().next())
    }
    
    pub async fn update_friend_status(&self, friend_id: &str, status: i32) -> Result<(), Error> {
        let sql = "UPDATE friend SET status = $status WHERE id = $friend_id";
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.client
            .query(sql)
            .bind(("friend_id", friend_id))
            .bind(("status", status))
            .await?
            .take(0)?;
        Ok(())
    }
    
    pub async fn update_friend_remark(&self, friend_id: &str, remark: Option<String>) -> Result<(), Error> {
        let sql = "UPDATE friend SET remark = $remark WHERE id = $friend_id";
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.client
            .query(sql)
            .bind(("friend_id", friend_id))
            .bind(("remark", remark))
            .await?
            .take(0)?;
        Ok(())
    }
    
    pub async fn delete_friend(&self, friend_id: &str) -> Result<(), Error> {
        let sql = "DELETE friend WHERE id = $friend_id";
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.client
            .query(sql)
            .bind(("friend_id", friend_id))
            .await?
            .take(0)?;
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
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.client
            .query(sql)
            .bind(("group_id", group_id))
            .bind(("user_id", user_id))
            .bind(("role", role))
            .await?
            .take(0)?;
        Ok(())
    }
    
    // 获取群组申请信息
    pub async fn get_group_apply_by_id(&self, apply_id: &str) -> Result<Option<GroupApply>, Error> {
        let sql = "SELECT * FROM group_apply WHERE id = $apply_id";
        
        let mut response = self.client
            .query(sql)
            .bind(("apply_id", apply_id))
            .await?;
        let applies: Vec<GroupApply> = response.take(0)?;
        
        Ok(applies.into_iter().next())
    }
    
    // 文件相关方法
    pub async fn create_file(&self, file: &ChatFile) -> Result<(), Error> {
        let sql = "CREATE chat_file CONTENT $data";
        
        let _: Option<Vec<surrealdb::sql::Value>> = self.client
            .query(sql)
            .bind(("data", file.clone()))
            .await?
            .take(0)?;
        Ok(())
    }
    
    pub async fn get_file_by_id(&self, file_id: &str) -> Result<Option<ChatFile>, Error> {
        let sql = "SELECT * FROM chat_file WHERE id = $file_id";
        
        let mut response = self.client
            .query(sql)
            .bind(("file_id", file_id))
            .await?;
        let files: Vec<ChatFile> = response.take(0)?;
        
        Ok(files.into_iter().next())
    }
}
