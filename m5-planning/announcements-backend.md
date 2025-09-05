# Notifications and Communications Backend

This document defines the comprehensive backend system for user notifications, announcements, and communication preferences. The system supports both broadcast announcements (visible to all users) and targeted notifications (specific to individual users or user groups).

## File Locations

### API Layer

- API endpoints: `backend/upwood/src/api/notifications.rs`
- Update: `backend/upwood/src/api/mod.rs` to include notifications module

### Database Layer

- Database Models: `backend/shared/src/db/notifications.rs`
- Schema: `backend/shared/src/schema.rs`

### Email Service Integration

- Email processor: `backend/shared/src/services/email_notifications.rs`
- AWS SES integration for notification emails

---

# PART I: DATABASE SCHEMA

## Core Notifications Tables

### notifications

```sql path=null start=null
CREATE TABLE notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    notification_type VARCHAR(50) NOT NULL CHECK (
        notification_type IN (
            'system_announcement', 'bond_available', 'investment_confirmed', 
            'investment_failed', 'yield_distributed', 'bond_matured', 
            'bond_status_updated', 'kyc_status_changed', 'marketing_update'
        )
    ),
    target_type VARCHAR(20) NOT NULL CHECK (target_type IN ('all_users', 'specific_user', 'user_group')),
    target_user_id TEXT, -- User ID when target_type = 'specific_user'
    target_criteria JSONB, -- Additional targeting criteria for user_group
    
    -- Metadata
    bond_metadata_id UUID, -- Reference to bond when notification is bond-related
    related_entity_type VARCHAR(50), -- 'bond', 'investment', 'yield', etc.
    related_entity_id TEXT, -- ID of related entity
    action_url TEXT, -- Optional deep link for notification action
    
    -- Status and timing
    priority INTEGER NOT NULL DEFAULT 1 CHECK (priority BETWEEN 1 AND 5), -- 1=low, 5=critical
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    effective_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE,
    
    -- Audit
    created_by_admin_id TEXT, -- Admin who created the notification (null for system-generated)
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Indexes for efficient queries
CREATE INDEX idx_notifications_target_user ON notifications(target_user_id, effective_date DESC) WHERE target_type = 'specific_user';
CREATE INDEX idx_notifications_all_users ON notifications(effective_date DESC) WHERE target_type = 'all_users' AND is_active = true;
CREATE INDEX idx_notifications_type ON notifications(notification_type, effective_date DESC);
CREATE INDEX idx_notifications_bond ON notifications(bond_metadata_id) WHERE bond_metadata_id IS NOT NULL;
CREATE INDEX idx_notifications_priority ON notifications(priority DESC, effective_date DESC);
CREATE INDEX idx_notifications_active_effective ON notifications(is_active, effective_date) WHERE is_active = true;
```

### user_notification_status

```sql path=null start=null
CREATE TABLE user_notification_status (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    notification_id UUID NOT NULL REFERENCES notifications(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL,
    is_read BOOLEAN NOT NULL DEFAULT FALSE,
    read_at TIMESTAMP WITH TIME ZONE,
    is_dismissed BOOLEAN NOT NULL DEFAULT FALSE,
    dismissed_at TIMESTAMP WITH TIME ZONE,
    email_sent BOOLEAN NOT NULL DEFAULT FALSE,
    email_sent_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    UNIQUE(notification_id, user_id)
);

-- Indexes for user notification queries
CREATE INDEX idx_user_notification_status_user ON user_notification_status(user_id, is_read, created_at DESC);
CREATE INDEX idx_user_notification_status_unread ON user_notification_status(user_id, is_read, created_at DESC) WHERE is_read = false;
CREATE INDEX idx_user_notification_status_notification ON user_notification_status(notification_id);
CREATE INDEX idx_user_notification_status_email_pending ON user_notification_status(email_sent, created_at) WHERE email_sent = false;
```

### user_notification_preferences

```sql path=null start=null
CREATE TABLE user_notification_preferences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id TEXT NOT NULL UNIQUE,
    
    -- Notification delivery preferences
    email_notifications BOOLEAN NOT NULL DEFAULT TRUE,
    
    -- Content preferences
    investment_updates BOOLEAN NOT NULL DEFAULT TRUE, -- Bond status changes, yield distributions, etc.
    marketing_communications BOOLEAN NOT NULL DEFAULT TRUE, -- Marketing announcements, new opportunities
    
    -- System notifications (always enabled, non-configurable)
    -- investment_confirmations, kyc_status_changes are always sent
    
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_user_notification_preferences_user ON user_notification_preferences(user_id);
```

---

# PART II: API ENDPOINTS

## Admin Notification Management

### POST /admin/notifications (Admin)

Create a new notification or announcement

**Headers:**

- `Authorization: Bearer <admin_jwt_token>` (required)

**Input Parameters:**

```json
{
  "title": "New Project Available",
  "message": "Nordic Spruce Estate is now open for investment",
  "notification_type": "bond_available",
  "target_type": "all_users",
  "target_user_id": null,
  "bond_metadata_id": "bmd_123e4567-e89b-12d3-a456-426614174000",
  "action_url": "/bonds/bmd_123e4567-e89b-12d3-a456-426614174000",
  "priority": 3,
  "effective_date": "2024-03-15T10:30:00Z",
  "expires_at": "2024-12-31T23:59:59Z"
}
```

**Response:**

```json
{
  "notification_id": "not_123e4567-e89b-12d3-a456-426614174000",
  "created_at": "2024-03-15T10:30:00Z"
}
```

### POST /admin/notifications/targeted (Admin)

Create targeted notification for specific user

**Input Parameters:**

```json
{
  "title": "Investment Successful",
  "message": "Your investment in Baltic Pine Forest has been confirmed",
  "notification_type": "investment_confirmed",
  "target_type": "specific_user",
  "target_user_id": "user_456",
  "bond_metadata_id": "bmd_456e4567-e89b-12d3-a456-426614174000",
  "related_entity_type": "investment",
  "related_entity_id": "inv_789",
  "priority": 4
}
```

### PUT /admin/notifications/{notification_id} (Admin)

Update existing notification

**Input Parameters:** Same as POST create

### DELETE /admin/notifications/{notification_id} (Admin)

Delete notification (soft delete - sets is_active = false)

**Response:** 204 No Content

### GET /admin/notifications (Admin)

List all notifications with admin details

**Query Parameters:**

- `page` (number, optional, default: 1)
- `page_size` (number, optional, default: 20)
- `notification_type` (string, optional)
- `target_type` (string, optional)
- `include_inactive` (boolean, optional, default: false)

## Investor Notification Endpoints

### GET /notifications (Investor)

Get notifications for the authenticated user

**Headers:**

- `Authorization: Bearer <jwt_token>` (required)

**Query Parameters:**

- `page` (number, optional, default: 1)
- `page_size` (number, optional, default: 20)
- `unread_only` (boolean, optional, default: false)
- `notification_type` (string, optional)

**Response:**

```json
{
  "notifications": [
    {
      "notification_id": "not_123e4567-e89b-12d3-a456-426614174000",
      "title": "New Project Available",
      "message": "Nordic Spruce Estate is now open for investment",
      "notification_type": "bond_available",
      "priority": 3,
      "is_read": false,
      "action_url": "/bonds/bmd_123e4567-e89b-12d3-a456-426614174000",
      "effective_date": "2024-03-15T10:30:00Z",
      "bond_metadata_id": "bmd_123e4567-e89b-12d3-a456-426614174000",
      "is_new": true
    },
    {
      "notification_id": "not_456e4567-e89b-12d3-a456-426614174000",
      "title": "Investment Successful",
      "message": "Your investment in Baltic Pine Forest has been confirmed",
      "notification_type": "investment_confirmed",
      "priority": 4,
      "is_read": false,
      "effective_date": "2024-03-15T09:15:00Z",
      "bond_metadata_id": "bmd_456e4567-e89b-12d3-a456-426614174000",
      "is_new": true
    }
  ],
  "total_count": 25,
  "unread_count": 8,
  "page_info": {
    "current_page": 1,
    "total_pages": 2,
    "has_next_page": true,
    "has_previous_page": false
  }
}
```

### POST /notifications/mark-read (Investor)

Mark specific notifications as read

**Input Parameters:**

```json
{
  "notification_ids": [
    "not_123e4567-e89b-12d3-a456-426614174000",
    "not_456e4567-e89b-12d3-a456-426614174000"
  ]
}
```

**Response:**

```json
{
  "marked_read_count": 2,
  "success": true
}
```

### POST /notifications/mark-all-read (Investor)

Mark all visible notifications as read for the user

**Response:**

```json
{
  "marked_read_count": 8,
  "success": true
}
```

## Notification Preferences Endpoints

### GET /notifications/preferences (Investor)

Get user's notification preferences

**Response:**

```json
{
  "user_id": "user_456",
  "email_notifications": true,
  "investment_updates": true,
  "marketing_communications": false,
  "updated_at": "2024-03-15T10:30:00Z"
}
```

### PUT /notifications/preferences (Investor)

Update user's notification preferences

**Input Parameters:**

```json
{
  "email_notifications": true,
  "investment_updates": true,
  "marketing_communications": false
}
```

**Response:**

```json
{
  "success": true,
  "updated_at": "2024-03-15T10:30:00Z"
}
```

---

# PART III: SYSTEM INTEGRATION

## Automated Notification Triggers

Other backend systems integrate with the notifications system by creating notifications through service calls:

### Bond System Integration

```rust path=null start=null
// Example: When a new bond becomes available
pub async fn notify_bond_available(
    conn: &mut DbConn,
    bond_metadata_id: Uuid,
    bond_name: &str,
) -> Result<(), NotificationError> {
    let notification = NewNotification {
        title: "New Project Available".to_string(),
        message: format!("{} is now open for investment", bond_name),
        notification_type: NotificationType::BondAvailable,
        target_type: TargetType::AllUsers,
        bond_metadata_id: Some(bond_metadata_id),
        priority: 3,
        action_url: Some(format!("/bonds/{}", bond_metadata_id)),
        ..Default::default()
    };
    
    NotificationService::create_notification(conn, notification).await
}
```

### Investment System Integration

```rust path=null start=null
// Example: When investment is confirmed
pub async fn notify_investment_confirmed(
    conn: &mut DbConn,
    user_id: &str,
    bond_metadata_id: Uuid,
    bond_name: &str,
    investment_amount: Decimal,
) -> Result<(), NotificationError> {
    let notification = NewNotification {
        title: "Investment Successful".to_string(),
        message: format!("Your investment in {} has been confirmed", bond_name),
        notification_type: NotificationType::InvestmentConfirmed,
        target_type: TargetType::SpecificUser,
        target_user_id: Some(user_id.to_string()),
        bond_metadata_id: Some(bond_metadata_id),
        priority: 4,
        ..Default::default()
    };
    
    NotificationService::create_notification(conn, notification).await
}
```

## Email Notification Processing

### Background Email Processor

```rust path=null start=null
// Scheduled job that processes pending email notifications
pub async fn process_pending_email_notifications(
    conn: &mut DbConn,
    ses_client: &SesClient,
) -> Result<(), EmailProcessorError> {
    let pending_notifications = UserNotificationStatus::get_pending_emails(conn).await?;
    
    for status in pending_notifications {
        let user_prefs = UserNotificationPreferences::get_by_user_id(
            conn, 
            &status.user_id
        ).await?;
        
        // Check if user wants email notifications
        if !user_prefs.email_notifications {
            continue;
        }
        
        // Check content preferences
        let notification = Notification::get_by_id(conn, &status.notification_id).await?;
        if !should_send_notification_type(&user_prefs, &notification.notification_type) {
            continue;
        }
        
        // Send email via AWS SES
        let result = send_notification_email(
            ses_client,
            &status.user_id,
            &notification
        ).await;
        
        // Update email status
        match result {
            Ok(_) => {
                UserNotificationStatus::mark_email_sent(
                    conn,
                    &status.id
                ).await?;
            }
            Err(e) => {
                log::error!("Failed to send email notification: {:?}", e);
                // Could implement retry logic here
            }
        }
    }
    
    Ok(())
}
```

## Notification Service Interface

```rust path=null start=null
pub struct NotificationService;

impl NotificationService {
    pub async fn create_notification(
        conn: &mut DbConn,
        notification: NewNotification,
    ) -> Result<Notification, NotificationError> {
        // Create notification record
        let notification = Notification::create(conn, notification).await?;
        
        // Create user_notification_status records based on target_type
        match notification.target_type {
            TargetType::AllUsers => {
                // Create status records for all active users
                Self::create_status_for_all_users(conn, &notification.id).await?;
            },
            TargetType::SpecificUser => {
                if let Some(user_id) = &notification.target_user_id {
                    Self::create_status_for_user(conn, &notification.id, user_id).await?;
                }
            },
            TargetType::UserGroup => {
                // Implement group targeting logic based on target_criteria
                Self::create_status_for_user_group(conn, &notification).await?;
            }
        }
        
        Ok(notification)
    }
    
    pub async fn mark_notifications_read(
        conn: &mut DbConn,
        user_id: &str,
        notification_ids: &[Uuid],
    ) -> Result<usize, NotificationError> {
        UserNotificationStatus::mark_multiple_read(
            conn,
            user_id,
            notification_ids
        ).await
    }
}
```

---

# PART IV: DATABASE MODELS

## Diesel Models

### Notification Model

```rust path=null start=null
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = notifications)]
pub struct Notification {
    pub id: Uuid,
    pub title: String,
    pub message: String,
    pub notification_type: String,
    pub target_type: String,
    pub target_user_id: Option<String>,
    pub target_criteria: Option<Value>,
    pub bond_metadata_id: Option<Uuid>,
    pub related_entity_type: Option<String>,
    pub related_entity_id: Option<String>,
    pub action_url: Option<String>,
    pub priority: i32,
    pub is_active: bool,
    pub effective_date: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_by_admin_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = notifications)]
pub struct NewNotification {
    pub title: String,
    pub message: String,
    pub notification_type: String,
    pub target_type: String,
    pub target_user_id: Option<String>,
    pub target_criteria: Option<Value>,
    pub bond_metadata_id: Option<Uuid>,
    pub related_entity_type: Option<String>,
    pub related_entity_id: Option<String>,
    pub action_url: Option<String>,
    pub priority: i32,
    pub effective_date: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_by_admin_id: Option<String>,
}

// Implementation methods removed per WARP rules - will be implemented during coding phase
```

### UserNotificationStatus Model

```rust path=null start=null
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = user_notification_status)]
pub struct UserNotificationStatus {
    pub id: Uuid,
    pub notification_id: Uuid,
    pub user_id: String,
    pub is_read: bool,
    pub read_at: Option<DateTime<Utc>>,
    pub is_dismissed: bool,
    pub dismissed_at: Option<DateTime<Utc>>,
    pub email_sent: bool,
    pub email_sent_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = user_notification_status)]
pub struct NewUserNotificationStatus {
    pub notification_id: Uuid,
    pub user_id: String,
}

// Implementation methods removed per WARP rules - will be implemented during coding phase
```

### UserNotificationPreferences Model

```rust path=null start=null
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = user_notification_preferences)]
pub struct UserNotificationPreferences {
    pub id: Uuid,
    pub user_id: String,
    pub email_notifications: bool,
    pub investment_updates: bool,
    pub marketing_communications: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_notification_preferences)]
pub struct NewUserNotificationPreferences {
    pub user_id: String,
    pub email_notifications: bool,
    pub investment_updates: bool,
    pub marketing_communications: bool,
}

// Implementation methods removed per WARP rules - will be implemented during coding phase
```

---

# PART V: INTEGRATION WORKFLOWS

## Notification Types and Triggers

### System-Generated Notifications

These notifications are automatically created by other backend systems:

| Notification Type | Trigger | Target | Email Preference | Content Preference |
|-------------------|---------|---------|------------------|--------------------|
| `investment_confirmed` | Investment processed successfully | Specific user | Always sent | Always sent (critical) |
| `investment_failed` | Investment processing failed | Specific user | Always sent | Always sent (critical) |
| `yield_distributed` | Yield payment completed | Specific user | Check email_notifications | Check investment_updates |
| `bond_matured` | Bond reaches maturity | Bond investors | Check email_notifications | Check investment_updates |
| `bond_status_updated` | Bond status changes | Bond investors | Check email_notifications | Check investment_updates |
| `kyc_status_changed` | KYC verification status changes | Specific user | Always sent | Always sent (critical) |

### Admin-Created Notifications

| Notification Type | Purpose | Target | Email Preference | Content Preference |
|-------------------|---------|---------|------------------|--------------------|
| `bond_available` | New bond open for investment | All users | Check email_notifications | Check investment_updates |
| `system_announcement` | General platform announcements | All users or specific groups | Check email_notifications | Check marketing_communications |
| `marketing_update` | Marketing communications | All users or targeted groups | Check email_notifications | Check marketing_communications |

## Integration Points

### Bond Backend Integration

**When to create notifications:**

1. **New bond published** → `bond_available` notification to all users
2. **Bond status changes** → `bond_status_updated` to bond investors
3. **Bond reaches maturity** → `bond_matured` to bond investors

**Integration code location:** `backend/upwood/src/api/bonds.rs`

### Investment Processing Integration

**When to create notifications:**

1. **Investment confirmed** → `investment_confirmed` to specific user
2. **Investment failed** → `investment_failed` to specific user

**Integration code location:** `backend/events_listener/src/processors/bonds.rs`

### Yields Backend Integration

**When to create notifications:**

1. **Yield distributed** → `yield_distributed` to each recipient

**Integration code location:** `backend/upwood/src/api/yields.rs`

### Identity Registry Integration

**When to create notifications:**

1. **KYC status changes** → `kyc_status_changed` to specific user

**Integration code location:** `backend/upwood/src/api/identity.rs`

## Email Content Preference Logic

```rust path=null start=null
fn should_send_notification_type(
    prefs: &UserNotificationPreferences,
    notification_type: &str,
) -> bool {
    match notification_type {
        // Always send critical notifications
        "investment_confirmed" | "investment_failed" | "kyc_status_changed" => true,
        
        // Investment-related notifications
        "yield_distributed" | "bond_matured" | "bond_status_updated" | "bond_available" => {
            prefs.investment_updates
        },
        
        // Marketing communications
        "system_announcement" | "marketing_update" => {
            prefs.marketing_communications
        },
        
        // Default to allowing
        _ => true,
    }
}
```

## Performance Considerations

### Database Optimization

- **Indexed queries** for user notification lookups
- **Compound indexes** for common query patterns (user + read status + date)
- **Partial indexes** for active notifications and unread status
- **Automatic cleanup** of old dismissed notifications (configurable retention)

### Email Processing Optimization

- **Background job processing** to avoid blocking API responses
- **Batch email sending** to improve AWS SES efficiency
- **Rate limiting** to stay within AWS SES sending limits
- **Retry logic** with exponential backoff for failed emails
- **Dead letter queue** for persistently failing email notifications

### Notification Storage Optimization

- **Automatic expiration** of old notifications based on expires_at
- **Bulk operations** for marking notifications as read
- **Efficient targeting** for broadcast notifications to avoid creating millions of status records
- **Lazy loading** of notification status for all-user notifications

## Security Considerations

### Authentication and Authorization

- **Admin endpoints** require admin JWT validation
- **Investor endpoints** require user JWT validation
- **Cross-user access prevention** - users can only access their own notifications
- **Admin audit trail** for all notification management operations

### Data Privacy

- **Personal data handling** in notification content
- **GDPR compliance** for email notifications and preferences
- **Data retention policies** for notification history
- **User consent** for marketing communications

### Content Security

- **Input validation** for all notification content
- **HTML sanitization** for rich text content
- **XSS prevention** in notification display
- **Rate limiting** for notification creation to prevent spam

## AWS Infrastructure Requirements

### AWS SES Configuration

- **Verified domain** for sending notification emails
- **SES templates** for different notification types
- **Bounce and complaint handling** webhooks
- **Sending statistics** monitoring and alerting

### Background Job Processing

- **ECS scheduled tasks** for email processing
- **SQS queues** for notification processing pipeline
- **Dead letter queues** for failed notifications
- **CloudWatch monitoring** for job success/failure rates

### Database Scaling

- **Read replicas** for notification queries
- **Connection pooling** for high-concurrency access
- **Database monitoring** for slow queries
- **Index optimization** based on query patterns
