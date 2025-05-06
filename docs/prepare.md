# prepare

create a `.env` file, and add the following content:

```ini
# -----------------------------------------------------------------------------
# Database (PostgreSQL)
# -----------------------------------------------------------------------------
DATABASE_URL=postgresql://postgres:password@localhost:5432/axum_auth

# -----------------------------------------------------------------------------
# JSON Web Token Credentials
# -----------------------------------------------------------------------------
JWT_SECRET_KEY=my_ultra_secure_jwt_secret_key
JWT_MAX_AGE=60

# -----------------------------------------------------------------------------
# SMTP Server Settings
# -----------------------------------------------------------------------------
SMTP_SERVER=smtp.your-email-provider.com
SMTP_PORT=587                     # Common ports: 587 (TLS), 465 (SSL), 25 (non-secure)
SMTP_USERNAME=your_email@example.com
SMTP_PASSWORD=your_email_password
SMTP_FROM_ADDRESS=no-reply@yourdomain.com
```
