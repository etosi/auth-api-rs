use chrono::{DateTime, Utc};
use core::str;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::User;

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct SignUpUserDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,
    #[validate(length(
        min = 6,
        message = "Password is required and must be at least 6 characters"
    ))]
    pub password: String,

    #[validate(
        length(min = 1, message = "Confirm Password is required"),
        must_match(other = "password", message = "passwords do not match")
    )]
    #[serde(rename = "passwordConfirm")]
    pub password_confirm: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct SignInUserDto {
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,
    #[validate(length(
        min = 6,
        message = "Password is required and must be at least 6 characters"
    ))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct RequestQueryDto {
    #[validate(range(min = 1))]
    pub page: Option<usize>,
    #[validate(range(min = 1, max = 50))]
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterUserDto {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub verified: bool,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl FilterUserDto {
    pub fn filter_user(user: &User) -> Self {
        FilterUserDto {
            id: user.id.to_string(),
            name: user.name.to_owned(),
            email: user.email.to_owned(),
            verified: user.verified,
            role: user.role.to_string(),
            created_at: user.created_at.unwrap(),
            updated_at: user.updated_at.unwrap(),
        }
    }

    pub fn filter_users(user: &[User]) -> Vec<FilterUserDto> {
        user.iter().map(FilterUserDto::filter_user).collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub user: FilterUserDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDto {
    pub status: ResponseStatus,
    pub data: UserData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserListResponseDto {
    pub status: ResponseStatus,
    pub users: Vec<FilterUserDto>,
    pub results: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSignInResponseDto {
    pub status: ResponseStatus,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSignOutResponseDto {
    pub status: ResponseStatus,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct NameUpdateDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ChangeUserRoleDto {
    #[validate(custom(function = "validate_user_role"))]
    pub role: String,
}

fn validate_user_role(role: &String) -> Result<(), validator::ValidationError> {
    match role.to_ascii_lowercase().as_str() {
        "admin" | "normal" => Ok(()),
        _ => return Err(validator::ValidationError::new("Invalid user role")),
    }
}

#[derive(Debug, Validate, Default, Clone, Serialize, Deserialize)]
pub struct UserPasswordUpdateDto {
    #[validate(length(
        min = 6,
        message = "New password is required and must be at least 6 characters."
    ))]
    pub new_password: String,

    #[validate(must_match(other = "new_password", message = "New passwords do not match."))]
    pub new_password_confirm: String,

    #[validate(length(
        min = 6,
        message = "Old password is required and must be at least 6 characters."
    ))]
    pub old_password: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct VerifyEmailQueryDto {
    #[validate(length(min = 1, message = "Token is required."))]
    pub token: String,
}

#[derive(Deserialize, Serialize, Validate, Debug, Clone)]
pub struct ForgotPasswordRequestDto {
    #[validate(
        length(min = 1, message = "Email is required."),
        email(message = "Email is invalid.")
    )]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct ResetPasswordRequestDto {
    #[validate(length(min = 1, message = "Token is required."))]
    pub token: String,

    #[validate(length(
        min = 6,
        message = "New password is required and must be at least 6 characters."
    ))]
    pub new_password: String,

    #[validate(must_match(other = "new_password", message = "New passwords do not match."))]
    pub new_password_confirm: String,
}
