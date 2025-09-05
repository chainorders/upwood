# Background Jobs Processing System

This document defines the unified background job processing system with modular service architecture for notifications, blockchain event processing, maturity payments, and yield distributions.

**Status**: Master file - consolidated from multiple planning documents

## File Locations

### Core Job System
- Job Queue Service: `backend/shared/src/jobs/job_queue.rs`
- Job Workers: `backend/shared/src/jobs/workers.rs`
- Job Models: `backend/shared/src/db/job_queue.rs`
- Background Worker Binary: `backend/upwood/src/bin/background_worker.rs`

### Modular Services
- Jobs Processing Service: `backend/shared/src/jobs/processing_service.rs`
- Maturity Service: `backend/shared/src/jobs/maturity_service.rs`
- Yields Service: `backend/shared/src/jobs/yields_service.rs`
- Notifications Service: `backend/shared/src/jobs/notifications_service.rs`

### Event Processing
- Event Tracking: `backend/shared/src/db/processed_events.rs`

---

# PART I: MODULAR SERVICE ARCHITECTURE

## JobsProcessingService Framework

The background worker uses a modular architecture where different services can be plugged in:

```rust path=null start=null
// Main background worker binary structure
let jobs_processing_service = JobsProcessingService::new(db_pool, job_queue);

// Add individual services
jobs_processing_service.add(MaturityService::new());
jobs_processing_service.add(YieldsService::new());
jobs_processing_service.add(NotificationsService::new());

// Start unified processing (never completes)
jobs_processing_service.run().await;
```

### Service Interface

```rust path=null start=null
#[async_trait]
pub trait JobService {
    fn job_types(&self) -> Vec<JobType>;
    async fn process_job(&self, job: &Job, context: &JobContext) -> JobResult;
    fn name(&self) -> &str;
}

pub struct JobContext {
    pub db_pool: DbPool,
    pub job_queue: JobQueue,
    pub email_service: EmailService,
    pub blockchain_service: BlockchainService,
}
```

### Background Worker Binary Implementation

```rust path=null start=null
// backend/upwood/src/bin/background_worker.rs
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_pool = create_db_pool().await?;
    let job_queue = JobQueue::new(db_pool.clone());
    
    let mut jobs_processing_service = JobsProcessingService::new(db_pool, job_queue);
    
    // Add all services
    jobs_processing_service.add(MaturityService::new());
    jobs_processing_service.add(YieldsService::new());
    jobs_processing_service.add(NotificationsService::new());
    
    info!("Starting unified background worker with all services...");
    
    // This never completes - runs indefinitely
    jobs_processing_service.run().await;
}
```

### Container Architecture

#### Final 3-Container Setup
1. **api-server**: REST API endpoints and web application logic
2. **events-listener**: Blockchain event processing and database persistence  
3. **background-worker**: All background job processing (unified service)

---

# PART II: JOB QUEUE SYSTEM

## PostgreSQL-Based Job Queue

### job_queue Table Schema
- **ACID Guarantees**: PostgreSQL-based job queue with transactional safety
- **Atomic Job Processing**: SELECT FOR UPDATE SKIP LOCKED prevents worker conflicts
- **Job Types**: Comprehensive enum covering all background processing needs
- **Job Statuses**: pending, processing, completed, failed, retrying
- **Retry Logic**: Exponential backoff with configurable max_retries
- **Worker Isolation**: processed_by field tracks which worker handles each job

### Core Job Types

#### Bond Management Jobs
- **BondInvestment**: Process blockchain investment transactions
- **BondClaim**: Handle bond token claims for successful bonds
- **BondRefund**: Process investment refunds for failed bonds

#### Maturity Processing Jobs
- **MaturityPayment**: Process bond maturity payments (main orchestrator job)
- **MaturityLiquidityCheck**: Verify cloud wallet has sufficient PLT for all maturity payments
- **MaturityTokenBurn**: Burn bond tokens for whitelisted investors (Phase 1)
- **MaturityPLTTransfer**: Transfer PLT tokens to investors after successful token burn (Phase 2)

#### Yield Processing Jobs
- **YieldDistribution**: Process yield distribution jobs triggered by admin (main orchestrator job)
- **YieldCalculation**: Calculate yield amounts based on token holdings and formulas
- **YieldPayment**: Process individual yield payments to investors

#### Notification Jobs
- **NotificationEventProcessor**: Scans blockchain events for notification triggers
- **NotificationFanoutProcessor**: Processes broadcast notifications (1-to-many)
- **NotificationIndividualProcessor**: Processes single user notifications with preference checks
- **EmailNotification**: Send individual email notifications

---

# PART III: MATURITY PROCESSING SERVICE

## MaturityService Implementation

### Service Overview
- **Purpose**: Process bond maturity payments when bonds reach maturity date
- **Trigger**: Admin API call to `/admin/bonds/{bond_metadata_id}/maturity/trigger`
- **Process**: Two-phase transaction (burn bond tokens → transfer PLT)
- **Requirements**: Cloud wallet liquidity check, investor whitelist validation

### MaturityService Structure

```rust path=null start=null
pub struct MaturityService {
    blockchain_client: BlockchainClient,
    identity_service: IdentityRegistryService,
}

#[async_trait]
impl JobService for MaturityService {
    fn job_types(&self) -> Vec<JobType> {
        vec![
            JobType::MaturityPayment,
            JobType::MaturityLiquidityCheck,
            JobType::MaturityTokenBurn,
            JobType::MaturityPLTTransfer,
        ]
    }
    
    async fn process_job(&self, job: &Job, context: &JobContext) -> JobResult {
        match job.job_type {
            JobType::MaturityPayment => self.process_maturity_payment(job, context).await,
            JobType::MaturityLiquidityCheck => self.check_liquidity(job, context).await,
            JobType::MaturityTokenBurn => self.burn_tokens(job, context).await,
            JobType::MaturityPLTTransfer => self.transfer_plt(job, context).await,
            _ => Err(JobError::UnsupportedJobType),
        }
    }
    
    fn name(&self) -> &str { "MaturityService" }
}
```

### Maturity Processing Flow

#### 1. MaturityPayment (Main Orchestrator Job)
```rust path=null start=null
async fn process_maturity_payment(&self, job: &Job, context: &JobContext) -> JobResult {
    let payload: MaturityPayload = serde_json::from_value(job.payload.clone())?;
    
    // Get all bond investors with postsale balances
    let investors = self.get_bond_investors(&payload.bond_metadata_id, context).await?;
    
    // Calculate total PLT required
    let total_plt_required = investors.iter()
        .map(|inv| inv.postsale_balance * payload.face_value_per_token)
        .sum::<Decimal>();
    
    // Create liquidity check job
    context.job_queue.enqueue_job(JobType::MaturityLiquidityCheck, json!({
        "maturity_job_id": payload.maturity_job_id,
        "total_plt_required": total_plt_required,
        "face_value_per_token": payload.face_value_per_token,
        "bond_metadata_id": payload.bond_metadata_id,
        "investors": investors
    }))?;
    
    Ok(())
}
```

#### 2. Liquidity Check and Token Burning
```rust path=null start=null
async fn check_liquidity(&self, job: &Job, context: &JobContext) -> JobResult {
    let payload: LiquidityCheckPayload = serde_json::from_value(job.payload.clone())?;
    
    // Check cloud wallet balance
    let wallet_balance = self.blockchain_client.get_plt_balance().await?;
    
    if wallet_balance < payload.total_plt_required {
        return Err(JobError::InsufficientLiquidity(format!(
            "Required: {}, Available: {}", 
            payload.total_plt_required, 
            wallet_balance
        )));
    }
    
    // Create token burn job for whitelisted investors
    context.job_queue.enqueue_job(JobType::MaturityTokenBurn, json!({
        "maturity_job_id": payload.maturity_job_id,
        "face_value_per_token": payload.face_value_per_token,
        "bond_metadata_id": payload.bond_metadata_id,
        "investors": payload.investors
    }))?;
    
    Ok(())
}

async fn burn_tokens(&self, job: &Job, context: &JobContext) -> JobResult {
    let payload: TokenBurnPayload = serde_json::from_value(job.payload.clone())?;
    
    for investor in &payload.investors {
        // Check whitelist status via identity registry
        if !self.identity_service.is_whitelisted(&investor.address).await? {
            tracing::warn!("Skipping non-whitelisted investor: {}", investor.address);
            continue;
        }
        
        // Burn bond tokens (Phase 1)
        match self.blockchain_client.burn_bond_tokens(
            &payload.bond_contract,
            &investor.address,
            investor.postsale_balance,
        ).await {
            Ok(tx_hash) => {
                // Create PLT transfer job (Phase 2)
                context.job_queue.enqueue_job(JobType::MaturityPLTTransfer, json!({
                    "maturity_job_id": payload.maturity_job_id,
                    "investor_address": investor.address,
                    "plt_amount": investor.postsale_balance * payload.face_value_per_token,
                    "burn_tx_hash": tx_hash
                }))?;
            }
            Err(e) => {
                tracing::error!("Token burn failed for {}: {}", investor.address, e);
                // Continue with other investors
            }
        }
    }
    
    Ok(())
}
```

#### 3. PLT Transfer (Phase 2)
```rust path=null start=null
async fn transfer_plt(&self, job: &Job, context: &JobContext) -> JobResult {
    let payload: PLTTransferPayload = serde_json::from_value(job.payload.clone())?;
    
    // Transfer PLT tokens to investor
    match self.blockchain_client.transfer_plt_tokens(
        &payload.investor_address,
        payload.plt_amount,
    ).await {
        Ok(tx_hash) => {
            // Update maturity payment record
            self.update_maturity_payment_record(
                &payload.maturity_job_id,
                &payload.investor_address,
                &payload.burn_tx_hash,
                &tx_hash,
                "completed",
                context,
            ).await?;
            
            // Create notification for successful maturity payment
            context.job_queue.enqueue_job(JobType::NotificationIndividualProcessor, json!({
                "user_id": payload.investor_address,
                "notification_type": "bond_matured",
                "title": "Bond Maturity Payment Completed",
                "message": format!("Your maturity payment of {} PLT has been processed", payload.plt_amount),
                "priority": 4
            }))?;
        }
        Err(e) => {
            tracing::error!("PLT transfer failed for {}: {}", payload.investor_address, e);
            return Err(JobError::PLTTransferFailed(e.to_string()));
        }
    }
    
    Ok(())
}
```

---

# PART IV: YIELDS PROCESSING SERVICE

## YieldsService Implementation

### Service Overview
- **Purpose**: Process yield distributions to bond token holders
- **Trigger**: Admin API call to `/admin/yields/{yield_config_id}/trigger`
- **Calculations**: Fixed and variable interest formulas with holding period calculations
- **Features**: Token ID timestamp conversion, holding period precision, PLT token distribution

### YieldsService Structure

```rust path=null start=null
pub struct YieldsService {
    blockchain_client: BlockchainClient,
    yield_calculator: YieldCalculator,
}

#[async_trait]
impl JobService for YieldsService {
    fn job_types(&self) -> Vec<JobType> {
        vec![
            JobType::YieldDistribution,
            JobType::YieldCalculation,
            JobType::YieldPayment,
        ]
    }
    
    async fn process_job(&self, job: &Job, context: &JobContext) -> JobResult {
        match job.job_type {
            JobType::YieldDistribution => self.process_distribution(job, context).await,
            JobType::YieldCalculation => self.calculate_yields(job, context).await,
            JobType::YieldPayment => self.process_payment(job, context).await,
            _ => Err(JobError::UnsupportedJobType),
        }
    }
    
    fn name(&self) -> &str { "YieldsService" }
}
```

### Yield Processing Flow

#### 1. YieldDistribution (Main Orchestrator Job)
```rust path=null start=null
async fn process_distribution(&self, job: &Job, context: &JobContext) -> JobResult {
    let payload: YieldDistributionPayload = serde_json::from_value(job.payload.clone())?;
    
    // Get yield configuration from database
    let yield_config = self.get_yield_config(&payload.yield_config_id, context).await?;
    
    // Get bond token holders with detailed token information
    let token_holders = self.get_token_holders_with_details(
        &yield_config.bond_metadata_id, 
        context
    ).await?;
    
    // Create calculation jobs for each investor
    for holder in token_holders {
        context.job_queue.enqueue_job(JobType::YieldCalculation, json!({
            "distribution_job_id": payload.distribution_job_id,
            "yield_config_id": payload.yield_config_id,
            "investor_address": holder.address,
            "token_holdings": holder.tokens, // Array of {token_id, balance}
            "yield_config": yield_config
        }))?;
    }
    
    Ok(())
}
```

#### 2. Yield Calculation with Token ID Timestamps
```rust path=null start=null
async fn calculate_yields(&self, job: &Job, context: &JobContext) -> JobResult {
    let payload: YieldCalculationPayload = serde_json::from_value(job.payload.clone())?;
    
    let mut total_yield = Decimal::ZERO;
    let mut calculation_details = Vec::new();
    
    // Process each token holding individually
    for token_holding in payload.token_holdings {
        // Convert token ID to mint date (FR-BT-4: days since Unix epoch)
        let mint_date = self.token_id_to_date(token_holding.token_id);
        
        // Calculate holding period for this specific token
        let holding_period = self.calculate_holding_period(
            mint_date,
            payload.yield_config.interest_period_start,
            payload.yield_config.interest_period_end,
        );
        
        // Calculate yield based on yield type
        let token_yield = match payload.yield_config.yield_type.as_str() {
            "fixed" => self.yield_calculator.calculate_fixed_yield(
                payload.yield_config.face_value,
                payload.yield_config.coupon_rate,
                holding_period,
                365, // days in year
            ),
            "variable" => self.yield_calculator.calculate_variable_yield(
                payload.yield_config.profits_before_tax.unwrap_or_default(),
                payload.yield_config.variable_rate.unwrap_or_default(),
                holding_period,
                payload.yield_config.total_interest_period_days,
            ),
            _ => return Err(JobError::InvalidYieldType),
        };
        
        let token_total = token_yield * token_holding.balance;
        total_yield += token_total;
        
        calculation_details.push(json!({
            "token_id": token_holding.token_id,
            "balance": token_holding.balance,
            "mint_date": mint_date,
            "holding_period_days": holding_period,
            "yield_per_token": token_yield,
            "total_yield": token_total
        }));
    }
    
    // Create payment job with calculated amounts
    context.job_queue.enqueue_job(JobType::YieldPayment, json!({
        "distribution_job_id": payload.distribution_job_id,
        "yield_config_id": payload.yield_config_id,
        "investor_address": payload.investor_address,
        "payment_amount": total_yield,
        "calculation_details": calculation_details
    }))?;
    
    Ok(())
}
```

#### 3. Token ID Conversion and Holding Period Calculation
```rust path=null start=null
// FR-BT-4: Convert token ID (u64 days since Unix epoch) to actual date
fn token_id_to_date(&self, token_id: u64) -> chrono::NaiveDate {
    let unix_epoch = chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
    unix_epoch + chrono::Duration::days(token_id as i64)
}

// FR-BT-4: Calculate precise holding period per token
fn calculate_holding_period(
    &self,
    token_mint_date: chrono::NaiveDate,
    interest_period_start: chrono::NaiveDate,
    interest_period_end: chrono::NaiveDate,
) -> i32 {
    // Holding period = interest_period_end - max(token_mint_date, interest_period_start)
    let effective_start = std::cmp::max(token_mint_date, interest_period_start);
    std::cmp::max(0, (interest_period_end - effective_start).num_days() as i32)
}
```

#### 4. Yield Payment Processing
```rust path=null start=null
async fn process_payment(&self, job: &Job, context: &JobContext) -> JobResult {
    let payload: YieldPaymentPayload = serde_json::from_value(job.payload.clone())?;
    
    // Transfer PLT tokens to investor
    match self.blockchain_client.transfer_plt_tokens(
        &payload.investor_address,
        payload.payment_amount,
    ).await {
        Ok(tx_hash) => {
            // Record payment in database
            self.record_yield_payment(
                &payload.distribution_job_id,
                &payload.yield_config_id,
                &payload.investor_address,
                payload.payment_amount,
                &tx_hash,
                &payload.calculation_details,
                context,
            ).await?;
            
            // Create notification for yield payment
            context.job_queue.enqueue_job(JobType::NotificationIndividualProcessor, json!({
                "user_id": payload.investor_address,
                "notification_type": "yield_distributed",
                "title": "Yield Payment Received",
                "message": format!("You have received {} PLT in yield payments", payload.payment_amount),
                "priority": 3
            }))?;
        }
        Err(e) => {
            tracing::error!("Yield payment failed for {}: {}", payload.investor_address, e);
            return Err(JobError::PaymentFailed(e.to_string()));
        }
    }
    
    Ok(())
}
```

---

# PART V: NOTIFICATIONS SERVICE

## NotificationsService Implementation

### Service Overview
- **Purpose**: Process notification events, fan-out broadcasts, and individual notifications
- **Event Processing**: Scans blockchain events every 30 seconds for user notifications
- **Fan-Out Processing**: Broadcast notifications to thousands of users efficiently
- **User Preferences**: Respects email and content preferences during processing

### NotificationsService Structure

```rust path=null start=null
pub struct NotificationsService {
    email_service: EmailService,
}

#[async_trait]
impl JobService for NotificationsService {
    fn job_types(&self) -> Vec<JobType> {
        vec![
            JobType::NotificationEventProcessor,
            JobType::NotificationFanoutProcessor,
            JobType::NotificationIndividualProcessor,
            JobType::EmailNotification,
        ]
    }
    
    async fn process_job(&self, job: &Job, context: &JobContext) -> JobResult {
        match job.job_type {
            JobType::NotificationEventProcessor => self.process_events(job, context).await,
            JobType::NotificationFanoutProcessor => self.process_fanout(job, context).await,
            JobType::NotificationIndividualProcessor => self.process_individual(job, context).await,
            JobType::EmailNotification => self.send_email(job, context).await,
            _ => Err(JobError::UnsupportedJobType),
        }
    }
    
    fn name(&self) -> &str { "NotificationsService" }
}
```

### Notification Processing Flow

#### 1. Event-Driven Notifications (Recurring Job)
```rust path=null start=null
async fn process_events(&self, job: &Job, context: &JobContext) -> JobResult {
    // Get unprocessed blockchain events
    let unprocessed_events = self.get_unprocessed_events(context).await?;
    
    for event in unprocessed_events {
        // Check if already processed (deduplication)
        if self.is_event_processed(&event.id, context).await? {
            continue;
        }
        
        // Create individual notification based on event type
        let notification_payload = match event.record_type.as_str() {
            "invest" => self.create_investment_notification(&event),
            "claim" => self.create_claim_notification(&event),
            "refund" => self.create_refund_notification(&event),
            _ => continue,
        };
        
        // Create individual notification job
        context.job_queue.enqueue_job(
            JobType::NotificationIndividualProcessor,
            notification_payload,
        )?;
        
        // Mark event as processed
        self.mark_event_processed(&event.id, context).await?;
    }
    
    // Schedule next event processing (30 seconds later)
    context.job_queue.enqueue_delayed_job(
        JobType::NotificationEventProcessor,
        json!({}),
        chrono::Utc::now() + chrono::Duration::seconds(30),
    )?;
    
    Ok(())
}
```

#### 2. Fan-Out Processing
```rust path=null start=null
async fn process_fanout(&self, job: &Job, context: &JobContext) -> JobResult {
    let payload: FanoutPayload = serde_json::from_value(job.payload.clone())?;
    
    // Get target users based on notification type and preferences
    let target_users = self.get_target_users(&payload.notification_type, context).await?;
    
    // Create individual notification jobs for each target user
    for user_id in target_users {
        context.job_queue.enqueue_job(JobType::NotificationIndividualProcessor, json!({
            "user_id": user_id,
            "notification_type": payload.notification_type,
            "title": payload.title,
            "message": payload.message,
            "bond_metadata_id": payload.bond_metadata_id,
            "priority": payload.priority
        }))?;
    }
    
    Ok(())
}
```

#### 3. Individual Notification with Preference Checking
```rust path=null start=null
async fn process_individual(&self, job: &Job, context: &JobContext) -> JobResult {
    let payload: IndividualNotificationPayload = serde_json::from_value(job.payload.clone())?;
    
    // Get user notification preferences
    let user_prefs = self.get_user_preferences(&payload.user_id, context).await?;
    
    // Check if user wants this type of notification
    if !self.should_send_notification(&user_prefs, &payload.notification_type) {
        return Ok(()); // Skip this notification
    }
    
    // Always create database notification record
    self.create_notification_record(&payload, context).await?;
    
    // Send email if user has email notifications enabled
    if user_prefs.email_notifications {
        context.job_queue.enqueue_job(JobType::EmailNotification, json!({
            "user_id": payload.user_id,
            "title": payload.title,
            "message": payload.message,
            "notification_type": payload.notification_type
        }))?;
    }
    
    Ok(())
}
```

---

# PART VI: INTEGRATION AND DEPLOYMENT

## Service Integration Points

### API Integration
- **Maturity Trigger**: `/admin/bonds/{bond_metadata_id}/maturity/trigger` creates MaturityPayment job
- **Yield Trigger**: `/admin/yields/{yield_config_id}/trigger` creates YieldDistribution job
- **Bond Announcements**: `/admin/bonds/{bond_metadata_id}/announce` creates NotificationFanoutProcessor job

### Database Integration
- **Existing Infrastructure**: Uses existing PostgreSQL database and Diesel ORM
- **New Tables**: job_queue, processed_events extend existing schema
- **Event Sources**: Reads from bond_investment_records, yield_configs, bonds_metadata

### Blockchain Integration
- **Cloud Wallet**: PLT token transfers for maturity and yield payments
- **Token Operations**: Bond token burning for maturity processing
- **Identity Registry**: Whitelist validation for payment eligibility

## Performance and Reliability

### Job Processing Characteristics
- **Atomic Processing**: SELECT FOR UPDATE SKIP LOCKED prevents conflicts
- **Retry Logic**: Exponential backoff for failed jobs with configurable max retries
- **Dead Letter Queue**: Failed jobs after max retries available for manual review
- **Concurrent Processing**: 4 concurrent jobs per worker container

### Error Handling
- **Service Isolation**: Failed jobs in one service don't affect other services
- **Transaction Safety**: Database rollbacks on job processing failures
- **Comprehensive Logging**: Detailed error tracking and job statistics
- **Graceful Degradation**: Email failures don't fail notification delivery

### Monitoring and Statistics
- **Real-time Metrics**: Job counts by status (pending, processing, completed, failed)
- **Service Health**: Individual service status and processing rates
- **Performance Tracking**: Job processing times and throughput metrics
- **Resource Monitoring**: Database connection usage and memory consumption

This modular background job system provides professional-grade processing capabilities with clear separation of concerns, comprehensive error handling, and efficient resource utilization.
