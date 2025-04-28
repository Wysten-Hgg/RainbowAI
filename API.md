# API Documentation

## User Authentication

### Register
- **Endpoint**: `/auth/register`
- **Method**: POST
- **Request Body**:
  ```json
  {
    "email": "user@example.com",
    "password": "password123",
    "username": "username",
    "invite_code": "ABC123" // 可选
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
- **Headers**: Authorization: Bearer {token}
- **Response**:
  - **200 OK**: Returns a new token.
  - **401 Unauthorized**: Invalid token.

### Logout
- **Endpoint**: `/auth/logout`
- **Method**: POST
- **Headers**: Authorization: Bearer {token}
- **Response**:
  - **200 OK**: Logout successful.

## User Management

### Get User Profile
- **Endpoint**: `/user/profile`
- **Method**: GET
- **Headers**: Authorization: Bearer {token}
- **Response**:
  - **200 OK**: Returns user profile details.
  - **401 Unauthorized**: Invalid token.

### Update User Profile
- **Endpoint**: `/user/profile`
- **Method**: PUT
- **Headers**: Authorization: Bearer {token}
- **Request Body**:
  ```json
  {
    "username": "new_username",
    "avatar": "avatar_url",
    "bio": "User biography"
  }
  ```
- **Response**:
  - **200 OK**: Profile updated successfully.
  - **401 Unauthorized**: Invalid token.

### Get User VIP Status
- **Endpoint**: `/user/vip`
- **Method**: GET
- **Headers**: Authorization: Bearer {token}
- **Response**:
  - **200 OK**: Returns VIP status details.
  - **401 Unauthorized**: Invalid token.

### Upgrade VIP
- **Endpoint**: `/user/vip/upgrade`
- **Method**: POST
- **Headers**: Authorization: Bearer {token}
- **Request Body**:
  ```json
  {
    "level": 1,
    "payment_method": "credit_card"
  }
  ```
- **Response**:
  - **200 OK**: VIP upgraded successfully.
  - **400 Bad Request**: Invalid level or payment method.
  - **401 Unauthorized**: Invalid token.

## AI Interaction

### Get AI List
- **Endpoint**: `/ai/list`
- **Method**: GET
- **Headers**: Authorization: Bearer {token}
- **Response**:
  - **200 OK**: Returns list of available AI characters.

### Get AI Details
- **Endpoint**: `/ai/{ai_id}`
- **Method**: GET
- **Headers**: Authorization: Bearer {token}
- **Response**:
  - **200 OK**: Returns AI character details.
  - **404 Not Found**: AI not found.

### Start Chat
- **Endpoint**: `/ai/{ai_id}/chat`
- **Method**: POST
- **Headers**: Authorization: Bearer {token}
- **Response**:
  - **200 OK**: Returns chat session details.
  - **401 Unauthorized**: Invalid token.
  - **404 Not Found**: AI not found.

### Send Message
- **Endpoint**: `/ai/chat/{session_id}/message`
- **Method**: POST
- **Headers**: Authorization: Bearer {token}
- **Request Body**:
  ```json
  {
    "content": "Hello, how are you?",
    "type": "text"
  }
  ```
- **Response**:
  - **200 OK**: Returns AI response.
  - **401 Unauthorized**: Invalid token.
  - **404 Not Found**: Session not found.

### Get Chat History
- **Endpoint**: `/ai/chat/{session_id}/history`
- **Method**: GET
- **Headers**: Authorization: Bearer {token}
- **Response**:
  - **200 OK**: Returns chat history.
  - **401 Unauthorized**: Invalid token.
  - **404 Not Found**: Session not found.

## Points and Wallet System

### Get Wallet Balance
- **Endpoint**: `/wallet/balance`
- **Method**: GET
- **Headers**: Authorization: Bearer {token}
- **Response**:
  - **200 OK**: Returns wallet balance details.
  - **401 Unauthorized**: Invalid token.

### Get Transaction History
- **Endpoint**: `/wallet/transactions`
- **Method**: GET
- **Headers**: Authorization: Bearer {token}
- **Query Parameters**:
  - `page`: Page number (default: 1)
  - `limit`: Items per page (default: 10)
  - `type`: Transaction type (optional)
- **Response**:
  - **200 OK**: Returns transaction history.
  - **401 Unauthorized**: Invalid token.

### Recharge Wallet
- **Endpoint**: `/wallet/recharge`
- **Method**: POST
- **Headers**: Authorization: Bearer {token}
- **Request Body**:
  ```json
  {
    "amount": 100,
    "currency": "CNY",
    "payment_method": "alipay"
  }
  ```
- **Response**:
  - **200 OK**: Returns payment details.
  - **400 Bad Request**: Invalid amount or payment method.
  - **401 Unauthorized**: Invalid token.

### Daily Check-in
- **Endpoint**: `/points/check-in`
- **Method**: POST
- **Headers**: Authorization: Bearer {token}
- **Response**:
  - **200 OK**: Check-in successful, returns points earned.
  - **400 Bad Request**: Already checked in today.
  - **401 Unauthorized**: Invalid token.

## Gift System

### Get Gift List
- **Endpoint**: `/gifts`
- **Method**: GET
- **Headers**: Authorization: Bearer {token}
- **Response**:
  - **200 OK**: Returns list of available gifts.
  - **401 Unauthorized**: Invalid token.

### Send Gift
- **Endpoint**: `/gifts/send`
- **Method**: POST
- **Headers**: Authorization: Bearer {token}
- **Request Body**:
  ```json
  {
    "ai_id": "ai_123",
    "gift_id": "gift_456",
    "quantity": 1,
    "message": "Enjoy this gift!"
  }
  ```
- **Response**:
  - **200 OK**: Gift sent successfully.
  - **400 Bad Request**: Invalid gift or insufficient balance.
  - **401 Unauthorized**: Invalid token.
  - **404 Not Found**: AI not found.

### Get Gift History
- **Endpoint**: `/gifts/history`
- **Method**: GET
- **Headers**: Authorization: Bearer {token}
- **Query Parameters**:
  - `page`: Page number (default: 1)
  - `limit`: Items per page (default: 10)
- **Response**:
  - **200 OK**: Returns gift history.
  - **401 Unauthorized**: Invalid token.

### Get Consecutive Gift Records
- **Endpoint**: `/gifts/consecutive`
- **Method**: GET
- **Headers**: Authorization: Bearer {token}
- **Response**:
  - **200 OK**: Returns consecutive gift records.
  - **401 Unauthorized**: Invalid token.

## Lucky Card System

### Get Lucky Cards
- **Endpoint**: `/lucky-cards`
- **Method**: GET
- **Headers**: Authorization: Bearer {token}
- **Response**:
  - **200 OK**: Returns list of user's lucky cards.
  - **401 Unauthorized**: Invalid token.

### Draw Lucky Card
- **Endpoint**: `/lucky-cards/draw`
- **Method**: POST
- **Headers**: Authorization: Bearer {token}
- **Response**:
  - **200 OK**: Returns drawn lucky card details.
  - **400 Bad Request**: No draws available.
  - **401 Unauthorized**: Invalid token.

### Use Lucky Card
- **Endpoint**: `/lucky-cards/use/{card_id}`
- **Method**: POST
- **Headers**: Authorization: Bearer {token}
- **Response**:
  - **200 OK**: Card used successfully.
  - **400 Bad Request**: Card already used or expired.
  - **401 Unauthorized**: Invalid token.
  - **404 Not Found**: Card not found.

## Shop System

### Get Shop Items
- **Endpoint**: `/shop/items`
- **Method**: GET
- **Headers**: Authorization: Bearer {token}
- **Response**:
  - **200 OK**: Returns list of shop items.
  - **401 Unauthorized**: Invalid token.

### Purchase Item
- **Endpoint**: `/shop/purchase`
- **Method**: POST
- **Headers**: Authorization: Bearer {token}
- **Request Body**:
  ```json
  {
    "item_id": "item_123",
    "quantity": 1
  }
  ```
- **Response**:
  - **200 OK**: Purchase successful.
  - **400 Bad Request**: Invalid item or insufficient balance.
  - **401 Unauthorized**: Invalid token.
  - **404 Not Found**: Item not found.

### Get Purchase History
- **Endpoint**: `/shop/history`
- **Method**: GET
- **Headers**: Authorization: Bearer {token}
- **Query Parameters**:
  - `page`: Page number (default: 1)
  - `limit`: Items per page (default: 10)
- **Response**:
  - **200 OK**: Returns purchase history.
  - **401 Unauthorized**: Invalid token.

## Coupon System

### Get User Coupons
- **Endpoint**: `/coupons`
- **Method**: GET
- **Headers**: Authorization: Bearer {token}
- **Response**:
  - **200 OK**: Returns list of user's coupons.
  - **401 Unauthorized**: Invalid token.

### Redeem Coupon
- **Endpoint**: `/coupons/redeem`
- **Method**: POST
- **Headers**: Authorization: Bearer {token}
- **Request Body**:
  ```json
  {
    "code": "COUPON123"
  }
  ```
- **Response**:
  - **200 OK**: Coupon redeemed successfully.
  - **400 Bad Request**: Invalid or expired coupon.
  - **401 Unauthorized**: Invalid token.

### Use Coupon
- **Endpoint**: `/coupons/use/{coupon_id}`
- **Method**: POST
- **Headers**: Authorization: Bearer {token}
- **Response**:
  - **200 OK**: Coupon used successfully.
  - **400 Bad Request**: Coupon already used or expired.
  - **401 Unauthorized**: Invalid token.
  - **404 Not Found**: Coupon not found.

## Invitation System

### Get Invitation Code
- **Endpoint**: `/invite/code`
- **Method**: GET
- **Headers**: Authorization: Bearer {token}
- **Response**:
  - **200 OK**: Returns user's invitation code.
  - **401 Unauthorized**: Invalid token.

### Get Invitation Statistics
- **Endpoint**: `/invite/stats`
- **Method**: GET
- **Headers**: Authorization: Bearer {token}
- **Response**:
  - **200 OK**: Returns invitation statistics.
  - **401 Unauthorized**: Invalid token.

### Get Invited Users
- **Endpoint**: `/invite/users`
- **Method**: GET
- **Headers**: Authorization: Bearer {token}
- **Query Parameters**:
  - `page`: Page number (default: 1)
  - `limit`: Items per page (default: 10)
- **Response**:
  - **200 OK**: Returns list of invited users.
  - **401 Unauthorized**: Invalid token.

## Promoter System

### Apply for Promoter
- **Endpoint**: `/promoter/apply`
- **Method**: POST
- **Headers**: Authorization: Bearer {token}
- **Request Body**:
  ```json
  {
    "promoter_type": "individual",
    "wallet_account": "wallet_address"
  }
  ```
- **Response**:
  - **200 OK**: Application submitted successfully.
  - **400 Bad Request**: Invalid promoter type.
  - **401 Unauthorized**: Invalid token.
  - **409 Conflict**: Already applied or is a promoter.

### Get Promoter Status
- **Endpoint**: `/promoter/status`
- **Method**: GET
- **Headers**: Authorization: Bearer {token}
- **Response**:
  - **200 OK**: Returns promoter status.
  - **401 Unauthorized**: Invalid token.
  - **404 Not Found**: Not a promoter.

### Upload Verification Document
- **Endpoint**: `/promoter/verification/document`
- **Method**: POST
- **Headers**: Authorization: Bearer {token}
- **Request Body**: Form data with file
- **Response**:
  - **200 OK**: Document uploaded successfully.
  - **400 Bad Request**: Invalid file format.
  - **401 Unauthorized**: Invalid token.
  - **404 Not Found**: Not a promoter.

### Sign Promoter Agreement
- **Endpoint**: `/promoter/agreement/sign`
- **Method**: POST
- **Headers**: Authorization: Bearer {token}
- **Response**:
  - **200 OK**: Agreement signed successfully.
  - **401 Unauthorized**: Invalid token.
  - **404 Not Found**: Not a promoter.

### Get Promotion Statistics
- **Endpoint**: `/promoter/stats`
- **Method**: GET
- **Headers**: Authorization: Bearer {token}
- **Response**:
  - **200 OK**: Returns promotion statistics.
  - **401 Unauthorized**: Invalid token.
  - **404 Not Found**: Not a promoter.

### Get Promotion Records
- **Endpoint**: `/promoter/records`
- **Method**: GET
- **Headers**: Authorization: Bearer {token}
- **Query Parameters**:
  - `page`: Page number (default: 1)
  - `limit`: Items per page (default: 10)
- **Response**:
  - **200 OK**: Returns promotion records.
  - **401 Unauthorized**: Invalid token.
  - **404 Not Found**: Not a promoter.

### Get Commission Records
- **Endpoint**: `/promoter/commissions`
- **Method**: GET
- **Headers**: Authorization: Bearer {token}
- **Query Parameters**:
  - `page`: Page number (default: 1)
  - `limit`: Items per page (default: 10)
  - `status`: Commission status (optional)
- **Response**:
  - **200 OK**: Returns commission records.
  - **401 Unauthorized**: Invalid token.
  - **404 Not Found**: Not a promoter.

### Update Payment Account
- **Endpoint**: `/promoter/payment-account`
- **Method**: PUT
- **Headers**: Authorization: Bearer {token}
- **Request Body**:
  ```json
  {
    "wallet_account": "new_wallet_address"
  }
  ```
- **Response**:
  - **200 OK**: Payment account updated successfully.
  - **401 Unauthorized**: Invalid token.
  - **404 Not Found**: Not a promoter.

### Request Withdrawal
- **Endpoint**: `/promoter/withdrawal/request`
- **Method**: POST
- **Headers**: Authorization: Bearer {token}
- **Request Body**:
  ```json
  {
    "amount": 100,
    "currency": "CNY",
    "payment_method": "bank_transfer",
    "account_info": "Bank account details"
  }
  ```
- **Response**:
  - **200 OK**: Withdrawal request submitted successfully.
  - **400 Bad Request**: Invalid amount or insufficient balance.
  - **401 Unauthorized**: Invalid token.
  - **404 Not Found**: Not a promoter.

### Get Withdrawal Requests
- **Endpoint**: `/promoter/withdrawal/requests`
- **Method**: GET
- **Headers**: Authorization: Bearer {token}
- **Response**:
  - **200 OK**: Returns withdrawal requests.
  - **401 Unauthorized**: Invalid token.
  - **404 Not Found**: Not a promoter.

## Admin API

### Admin User Management

#### Get All Users
- **Endpoint**: `/admin/users`
- **Method**: GET
- **Headers**: Authorization: Bearer {token}
- **Query Parameters**:
  - `page`: Page number (default: 1)
  - `limit`: Items per page (default: 10)
  - `search`: Search term (optional)
- **Response**:
  - **200 OK**: Returns list of users.
  - **401 Unauthorized**: Invalid token.
  - **403 Forbidden**: Not an admin.

#### Update User
- **Endpoint**: `/admin/users/{user_id}`
- **Method**: PUT
- **Headers**: Authorization: Bearer {token}
- **Request Body**:
  ```json
  {
    "username": "new_username",
    "email": "new_email@example.com",
    "status": "active",
    "role": "user"
  }
  ```
- **Response**:
  - **200 OK**: User updated successfully.
  - **401 Unauthorized**: Invalid token.
  - **403 Forbidden**: Not an admin.
  - **404 Not Found**: User not found.

### Admin AI Management

#### Create AI
- **Endpoint**: `/admin/ai`
- **Method**: POST
- **Headers**: Authorization: Bearer {token}
- **Request Body**:
  ```json
  {
    "name": "AI Name",
    "type": "companion",
    "description": "AI description",
    "avatar": "avatar_url",
    "personality": "AI personality",
    "system_prompt": "System prompt for AI"
  }
  ```
- **Response**:
  - **200 OK**: AI created successfully.
  - **401 Unauthorized**: Invalid token.
  - **403 Forbidden**: Not an admin.

#### Update AI
- **Endpoint**: `/admin/ai/{ai_id}`
- **Method**: PUT
- **Headers**: Authorization: Bearer {token}
- **Request Body**:
  ```json
  {
    "name": "New AI Name",
    "description": "New AI description",
    "avatar": "new_avatar_url",
    "personality": "New AI personality",
    "system_prompt": "New system prompt for AI"
  }
  ```
- **Response**:
  - **200 OK**: AI updated successfully.
  - **401 Unauthorized**: Invalid token.
  - **403 Forbidden**: Not an admin.
  - **404 Not Found**: AI not found.

#### Delete AI
- **Endpoint**: `/admin/ai/{ai_id}`
- **Method**: DELETE
- **Headers**: Authorization: Bearer {token}
- **Response**:
  - **200 OK**: AI deleted successfully.
  - **401 Unauthorized**: Invalid token.
  - **403 Forbidden**: Not an admin.
  - **404 Not Found**: AI not found.

### Admin Gift Management

#### Create Gift
- **Endpoint**: `/admin/gifts`
- **Method**: POST
- **Headers**: Authorization: Bearer {token}
- **Request Body**:
  ```json
  {
    "name": "Gift Name",
    "description": "Gift description",
    "image_url": "gift_image_url",
    "price": 100,
    "category": "virtual",
    "boost_value": 1.5,
    "is_active": true
  }
  ```
- **Response**:
  - **200 OK**: Gift created successfully.
  - **401 Unauthorized**: Invalid token.
  - **403 Forbidden**: Not an admin.

#### Update Gift
- **Endpoint**: `/admin/gifts/{gift_id}`
- **Method**: PUT
- **Headers**: Authorization: Bearer {token}
- **Request Body**:
  ```json
  {
    "name": "New Gift Name",
    "description": "New gift description",
    "image_url": "new_gift_image_url",
    "price": 150,
    "boost_value": 2.0,
    "is_active": true
  }
  ```
- **Response**:
  - **200 OK**: Gift updated successfully.
  - **401 Unauthorized**: Invalid token.
  - **403 Forbidden**: Not an admin.
  - **404 Not Found**: Gift not found.

#### Delete Gift
- **Endpoint**: `/admin/gifts/{gift_id}`
- **Method**: DELETE
- **Headers**: Authorization: Bearer {token}
- **Response**:
  - **200 OK**: Gift deleted successfully.
  - **401 Unauthorized**: Invalid token.
  - **403 Forbidden**: Not an admin.
  - **404 Not Found**: Gift not found.

### Admin Shop Management

#### Create Shop Item
- **Endpoint**: `/admin/shop/items`
- **Method**: POST
- **Headers**: Authorization: Bearer {token}
- **Request Body**:
  ```json
  {
    "name": "Item Name",
    "description": "Item description",
    "image_url": "item_image_url",
    "price": 200,
    "currency": "points",
    "stock": 100,
    "is_active": true
  }
  ```
- **Response**:
  - **200 OK**: Item created successfully.
  - **401 Unauthorized**: Invalid token.
  - **403 Forbidden**: Not an admin.

#### Update Shop Item
- **Endpoint**: `/admin/shop/items/{item_id}`
- **Method**: PUT
- **Headers**: Authorization: Bearer {token}
- **Request Body**:
  ```json
  {
    "name": "New Item Name",
    "description": "New item description",
    "image_url": "new_item_image_url",
    "price": 250,
    "stock": 150,
    "is_active": true
  }
  ```
- **Response**:
  - **200 OK**: Item updated successfully.
  - **401 Unauthorized**: Invalid token.
  - **403 Forbidden**: Not an admin.
  - **404 Not Found**: Item not found.

#### Delete Shop Item
- **Endpoint**: `/admin/shop/items/{item_id}`
- **Method**: DELETE
- **Headers**: Authorization: Bearer {token}
- **Response**:
  - **200 OK**: Item deleted successfully.
  - **401 Unauthorized**: Invalid token.
  - **403 Forbidden**: Not an admin.
  - **404 Not Found**: Item not found.

### Admin Promoter Management

#### Get Promoter Applications
- **Endpoint**: `/admin/promoters/applications`
- **Method**: GET
- **Headers**: Authorization: Bearer {token}
- **Query Parameters**:
  - `page`: Page number (default: 1)
  - `limit`: Items per page (default: 10)
  - `status`: Verification status (optional)
- **Response**:
  - **200 OK**: Returns promoter applications.
  - **401 Unauthorized**: Invalid token.
  - **403 Forbidden**: Not an admin.

#### Verify Promoter
- **Endpoint**: `/admin/promoters/{promoter_id}/verify`
- **Method**: POST
- **Headers**: Authorization: Bearer {token}
- **Request Body**:
  ```json
  {
    "approved": true,
    "commission_rate": 0.1,
    "renewal_rate": 0.05
  }
  ```
- **Response**:
  - **200 OK**: Promoter verified successfully.
  - **401 Unauthorized**: Invalid token.
  - **403 Forbidden**: Not an admin.
  - **404 Not Found**: Promoter not found.

#### Get Withdrawal Requests
- **Endpoint**: `/admin/promoters/withdrawals`
- **Method**: GET
- **Headers**: Authorization: Bearer {token}
- **Query Parameters**:
  - `page`: Page number (default: 1)
  - `limit`: Items per page (default: 10)
- **Response**:
  - **200 OK**: Returns withdrawal requests.
  - **401 Unauthorized**: Invalid token.
  - **403 Forbidden**: Not an admin.

#### Process Withdrawal Request
- **Endpoint**: `/admin/promoters/withdrawals/{request_id}/process`
- **Method**: POST
- **Headers**: Authorization: Bearer {token}
- **Request Body**:
  ```json
  {
    "approved": true,
    "transaction_id": "tx_123456"
  }
  ```
- **Response**:
  - **200 OK**: Withdrawal request processed successfully.
  - **401 Unauthorized**: Invalid token.
  - **403 Forbidden**: Not an admin.
  - **404 Not Found**: Withdrawal request not found.

### Admin Audit Logs

#### Get Audit Logs
- **Endpoint**: `/admin/audit-logs`
- **Method**: GET
- **Headers**: Authorization: Bearer {token}
- **Query Parameters**:
  - `page`: Page number (default: 1)
  - `limit`: Items per page (default: 10)
  - `user_id`: Filter by user ID (optional)
  - `action`: Filter by action type (optional)
  - `start_date`: Filter by start date (optional)
  - `end_date`: Filter by end date (optional)
- **Response**:
  - **200 OK**: Returns audit logs.
  - **401 Unauthorized**: Invalid token.
  - **403 Forbidden**: Not an admin.
