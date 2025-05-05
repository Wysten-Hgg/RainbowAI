use std::path::{Path, PathBuf};
use tokio::fs::{self, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;
use bytes::BytesMut;
use crate::models::chat::ChatFile;

pub struct FileStorage {
    base_path: PathBuf,
}

impl FileStorage {
    pub fn new(base_path: &str) -> Self {
        FileStorage {
            base_path: PathBuf::from(base_path),
        }
    }
    
    pub async fn save_file(
        &self, 
        user_id: &str,
        original_name: &str, 
        content_type: &str, 
        data: &[u8]
    ) -> Result<ChatFile, std::io::Error> {
        // 确定文件类型目录
        let type_dir = match content_type {
            t if t.starts_with("image/") => "images",
            t if t.starts_with("audio/") => "voices",
            t if t.starts_with("video/") => "videos",
            _ => "files",
        };
        
        // 获取文件扩展名
        let file_ext = Path::new(original_name)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_string();
        
        // 生成唯一文件名
        let save_name = format!("{}.{}", Uuid::new_v4(), file_ext);
        let save_dir = self.base_path.join(type_dir);
        
        // 确保目录存在
        fs::create_dir_all(&save_dir).await?;
        
        // 保存文件
        let save_path = save_dir.join(&save_name);
        let mut file = File::create(&save_path).await?;
        file.write_all(data).await?;
        
        // 创建文件记录
        let file_record = ChatFile::new(
            user_id.to_string(),
            original_name.to_string(),
            save_name,
            save_path.to_string_lossy().to_string(),
            file_ext,
            data.len() as i64,
            content_type.to_string(),
        );
        
        Ok(file_record)
    }
    
    pub async fn save_file_from_bytes(
        &self, 
        user_id: &str,
        original_name: &str, 
        content_type: &str, 
        data: &BytesMut
    ) -> Result<ChatFile, std::io::Error> {
        self.save_file(user_id, original_name, content_type, data).await
    }
    
    pub async fn get_file(&self, file_path: &str) -> Result<Vec<u8>, std::io::Error> {
        let path = Path::new(file_path);
        let mut file = File::open(path).await?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?;
        Ok(buffer)
    }
    
    pub async fn get_file_info(&self, file_path: &str) -> Result<(Vec<u8>, String), std::io::Error> {
        let content = self.get_file(file_path).await?;
        
        // 从文件路径获取内容类型
        let content_type = match Path::new(file_path).extension().and_then(|ext| ext.to_str()) {
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("png") => "image/png",
            Some("gif") => "image/gif",
            Some("mp3") => "audio/mpeg",
            Some("mp4") => "video/mp4",
            Some("pdf") => "application/pdf",
            Some("doc") | Some("docx") => "application/msword",
            Some("xls") | Some("xlsx") => "application/vnd.ms-excel",
            Some("ppt") | Some("pptx") => "application/vnd.ms-powerpoint",
            Some("zip") => "application/zip",
            Some("rar") => "application/x-rar-compressed",
            Some("txt") => "text/plain",
            _ => "application/octet-stream",
        }.to_string();
        
        Ok((content, content_type))
    }
    
    pub fn get_file_url(&self, file_path: &str) -> String {
        // 在实际应用中，这里应该返回可访问的URL
        // 简化起见，我们返回文件路径
        format!("/files/{}", Path::new(file_path).file_name().unwrap().to_string_lossy())
    }
    
    pub async fn delete_file(&self, file_path: &str) -> Result<(), std::io::Error> {
        let path = Path::new(file_path);
        if path.exists() {
            fs::remove_file(path).await?;
        }
        Ok(())
    }
}
