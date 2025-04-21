use lettre::{
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use std::env;
use anyhow::Result;
use crate::models::EmailVerification;

pub struct EmailService {
    smtp_transport: AsyncSmtpTransport<Tokio1Executor>,
    from_email: String,
    app_name: String,
    app_url: String,
}

impl EmailService {
    pub fn new() -> Result<Self> {
        // 从环境变量中读取SMTP配置
        let smtp_server = env::var("SMTP_SERVER").expect("SMTP_SERVER must be set");
        let smtp_port = env::var("SMTP_PORT").expect("SMTP_PORT must be set")
            .parse::<u16>().expect("SMTP_PORT must be a valid port number");
        let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
        let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");
        let from_email = env::var("FROM_EMAIL").expect("FROM_EMAIL must be set");
        let app_name = env::var("APP_NAME").unwrap_or_else(|_| "彩虹城".to_string());
        let app_url = env::var("APP_URL").expect("APP_URL must be set");

        // 创建SMTP传输
        let creds = Credentials::new(smtp_username, smtp_password);
        let smtp_transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_server)?
            .port(smtp_port)
            .credentials(creds)
            .build();

        Ok(Self {
            smtp_transport,
            from_email,
            app_name,
            app_url,
        })
    }

    // 发送验证邮件
    pub async fn send_verification_email(&self, verification: &EmailVerification) -> Result<()> {
        let subject = match verification.verification_type {
            crate::models::VerificationType::Registration => 
                format!("欢迎加入{}，请验证您的邮箱", self.app_name),
            crate::models::VerificationType::PasswordReset => 
                format!("{}密码重置验证码", self.app_name),
            crate::models::VerificationType::EmailChange => 
                format!("{}邮箱变更验证码", self.app_name),
        };

        let verification_link = format!(
            "{}/verify-email?id={}&code={}",
            self.app_url, verification.id, verification.code
        );

        let body = match verification.verification_type {
            crate::models::VerificationType::Registration => format!(
                "亲爱的用户，\n\n感谢您注册{}！请点击以下链接验证您的邮箱：\n\n{}\n\n或者输入验证码：{}\n\n此链接将在15分钟后失效。\n\n如果您没有注册账号，请忽略此邮件。\n\n祝好，\n{}团队",
                self.app_name, verification_link, verification.code, self.app_name
            ),
            crate::models::VerificationType::PasswordReset => format!(
                "亲爱的用户，\n\n您正在重置{}的密码。请使用以下验证码完成重置：\n\n{}\n\n此验证码将在15分钟后失效。\n\n如果您没有请求重置密码，请忽略此邮件。\n\n祝好，\n{}团队",
                self.app_name, verification.code, self.app_name
            ),
            crate::models::VerificationType::EmailChange => format!(
                "亲爱的用户，\n\n您正在变更{}的邮箱地址。请使用以下验证码完成变更：\n\n{}\n\n此验证码将在15分钟后失效。\n\n如果您没有请求变更邮箱，请忽略此邮件。\n\n祝好，\n{}团队",
                self.app_name, verification.code, self.app_name
            ),
        };

        let email = Message::builder()
            .from(format!("{} <{}>", self.app_name, self.from_email).parse()?)
            .to(verification.email.parse()?)
            .subject(subject)
            .body(body)?;

        self.smtp_transport.send(email).await?;
        Ok(())
    }
}
