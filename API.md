# API Documentation

## User Authentication

### Register
- **Endpoint**: `/auth/register`
- **Method**: POST
- **Request Body**:
  ```json
  {
    "email": "user@example.com",
    "password": "password123"
  }
  ```
- **Response**:
  - **200 OK**: Returns a JSON object with user details and authentication token.
  - **409 Conflict**: Email already exists.

### Login
- **Endpoint**: `/auth/login`
- **Method**: POST
- **Request Body**:
  ```json
  {
    "email": "user@example.com",
    "password": "password123"
  }
  ```
- **Response**:
  - **200 OK**: Returns a JSON object with user details and authentication token.
  - **401 Unauthorized**: Invalid credentials.

### Refresh Token
- **Endpoint**: `/auth/refresh`
- **Method**: POST
- **Response**:
  - **200 OK**: Returns a new authentication token.

## User Management

### Get Profile
- **Endpoint**: `/user/profile`
- **Method**: GET
- **Response**:
  - **200 OK**: Returns user profile details.

### Get Stats
- **Endpoint**: `/user/stats`
- **Method**: GET
- **Response**:
  - **200 OK**: Returns user statistics.

### Apply for Promoter
- **Endpoint**: `/user/apply-promoter`
- **Method**: POST
- **Response**:
  - **200 OK**: Successfully applied for promoter role.
  - **403 Forbidden**: User is not eligible to apply.

## AI Management

### Initiate AI
- **Endpoint**: `/ai/initiate`
- **Method**: POST
- **Request Body**:
  ```json
  {
    "ai_type": "companion"
  }
  ```
- **Response**:
  - **200 OK**: AI initiated successfully.
  - **403 Forbidden**: User cannot initiate this AI type.

### Check VIP Status
- **Endpoint**: `/ai/check-vip-status`
- **Method**: POST
- **Response**:
  - **200 OK**: Checks and updates VIP status if needed.

## Invite Management

### Create Invite
- **Endpoint**: `/invite/create`
- **Method**: POST
- **Response**:
  - **200 OK**: Invite created successfully.
  - **500 Internal Server Error**: Failed to create invite.

### Use Invite
- **Endpoint**: `/invite/use`
- **Method**: POST
- **Request Body**:
  ```json
  {
    "code": "invite_code"
  }
  ```
- **Response**:
  - **200 OK**: Invite used successfully.
  - **410 Gone**: Invite is no longer valid.

## Coupon Management

### Get My Coupons
- **Endpoint**: `/coupon/my`
- **Method**: GET
- **Response**:
  - **200 OK**: Returns a list of active coupons owned by the user.
  - **500 Internal Server Error**: Failed to retrieve coupons.

### Redeem Coupon
- **Endpoint**: `/coupon/redeem`
- **Method**: POST
- **Request Body**:
  - `coupon_id`: ID of the coupon to redeem.
- **Response**:
  - **200 OK**: Coupon redeemed successfully.
  - **403 Forbidden**: Coupon cannot be redeemed by the user.
  - **404 Not Found**: Coupon not found.
  - **500 Internal Server Error**: Failed to redeem coupon.

### Transfer Coupon
- **Endpoint**: `/coupon/transfer`
- **Method**: POST
- **Request Body**:
  - `coupon_id`: ID of the coupon to transfer.
  - `new_owner_id`: ID of the new owner.
- **Response**:
  - **200 OK**: Coupon transferred successfully.
  - **403 Forbidden**: Coupon cannot be transferred by the user.
  - **404 Not Found**: Coupon not found.
  - **500 Internal Server Error**: Failed to transfer coupon.

### Issue Coupon (Admin)
- **Endpoint**: `/coupon/issue/admin`
- **Method**: POST
- **Request Body**:
  - `coupons`: List of coupons to issue.
- **Response**:
  - **200 OK**: Coupons issued successfully.
  - **403 Forbidden**: User does not have permission to issue coupons.
  - **500 Internal Server Error**: Failed to issue coupons.

## Admin Management

### Update User Role
- **Endpoint**: `/admin/user/role`
- **Method**: POST
- **Response**:
  - **200 OK**: User role updated successfully.
  - **403 Forbidden**: User does not have permission to update roles.

### View Audit Logs
- **Endpoint**: `/admin/audit-logs`
- **Method**: GET
- **Response**:
  - **200 OK**: Returns a list of audit logs.
  - **500 Internal Server Error**: Failed to retrieve logs.

### Set VIP Configuration (Admin)
- **Endpoint**: `/admin/set_vip_config`
- **Method**: POST
- **Request Body**:
  ```json
  {
    "vip_level": "VIP level name",
    "config": {
      "key": "value"
    }
  }
  ```
- **Response**:
  - **200 OK**: VIP configuration updated successfully.
  - **403 Forbidden**: User does not have permission to update VIP configuration.
  - **500 Internal Server Error**: Failed to update VIP configuration.
