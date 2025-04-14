-- Create User table
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD id ON user TYPE string ASSERT $value != NONE;
DEFINE FIELD email ON user TYPE string ASSERT is::email($value);
DEFINE FIELD password_hash ON user TYPE string ASSERT $value != NONE;
DEFINE FIELD frontend_roles ON user TYPE array;
DEFINE FIELD backend_roles ON user TYPE array;
DEFINE FIELD vip_level ON user TYPE string;
DEFINE FIELD ai_partner_count ON user TYPE int;
DEFINE FIELD daily_chat_count ON user TYPE int;
DEFINE FIELD daily_lio_count ON user TYPE int;
DEFINE FIELD invite_code ON user TYPE option<string>;
DEFINE FIELD created_at ON user TYPE int;
DEFINE FIELD updated_at ON user TYPE int;

-- Create Invite table
DEFINE TABLE invite SCHEMAFULL;
DEFINE FIELD code ON invite TYPE string ASSERT $value != NONE;
DEFINE FIELD used_by ON invite TYPE array;
DEFINE FIELD creator_id ON invite TYPE string ASSERT $value != NONE;
DEFINE FIELD usage_limit ON invite TYPE int;
DEFINE FIELD expires_at ON invite TYPE int;
DEFINE FIELD created_at ON invite TYPE int;
DEFINE FIELD updated_at ON invite TYPE int;

-- Create AI table
DEFINE TABLE ai SCHEMAFULL;
DEFINE FIELD id ON ai TYPE string ASSERT $value != NONE;
DEFINE FIELD type ON ai TYPE string;
DEFINE FIELD status ON ai TYPE string;
DEFINE FIELD color_slot ON ai TYPE string;
DEFINE FIELD user_id ON ai TYPE string ASSERT $value != NONE;
DEFINE FIELD created_at ON ai TYPE int;
DEFINE FIELD updated_at ON ai TYPE int;

-- Create AuditLog table
DEFINE TABLE audit_log SCHEMAFULL;
DEFINE FIELD id ON audit_log TYPE string ASSERT $value != NONE;
DEFINE FIELD action ON audit_log TYPE string;
DEFINE FIELD user_id ON audit_log TYPE string ASSERT $value != NONE;
DEFINE FIELD created_at ON audit_log TYPE int;

-- Create EmailVerification table
DEFINE TABLE email_verification SCHEMAFULL;
DEFINE FIELD id ON email_verification TYPE string ASSERT $value != NONE;
DEFINE FIELD email ON email_verification TYPE string ASSERT is::email($value);
DEFINE FIELD code ON email_verification TYPE string;
DEFINE FIELD verification_type ON email_verification TYPE string;
DEFINE FIELD expires_at ON email_verification TYPE int;
DEFINE FIELD used ON email_verification TYPE bool;
DEFINE FIELD created_at ON email_verification TYPE int;
