use super::sendmail::send_email;

pub async fn send_verification_email(
    to_email: &str,
    username: &str,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let subject = "邮箱验证";
    let template_path = "src/mail/templates/verification-email.html";
    let base_url = "http://localhost:8000/api/auth/verify";
    let verification_link = combine_url_token(base_url, token);
    let placeholders = vec![
        ("{{username}}".to_string(), username.to_string()),
        ("{{verification_link}}".to_string(), verification_link),
    ];

    send_email(to_email, subject, template_path, &placeholders).await
}

fn combine_url_token(base_url: &str, token: &str) -> String {
    format!("{}?token={}", base_url, token)
}

pub async fn send_welcome_email(
    to_email: &str,
    username: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let subject = "欢迎使用";
    let template_path = "src/mail/templates/welcome-email.html";
    let placeholders = vec![("{{username}}".to_string(), username.to_string())];

    send_email(to_email, subject, template_path, &placeholders).await
}

pub async fn send_forgot_password_email(
    to_email: &str,
    username: &str,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let subject = "重置密码";
    let template_path = "src/mail/templates/reset-password-email.html";
    let base_url = "http://localhost:5173/reset-password";
    let reset_link = combine_url_token(base_url, token);
    let placeholders = vec![
        ("{{username}}".to_string(), username.to_string()),
        ("{{reset_link}}".to_string(), reset_link.to_string()),
    ];

    send_email(to_email, subject, template_path, &placeholders).await
}
