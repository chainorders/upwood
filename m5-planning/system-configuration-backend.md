# System Configuration Backend API (FR-AD-5)

This document defines the backend API for displaying system configuration information in the admin portal, including module refs, contract addresses, and environment variables.

## API Layer

### System Configuration Endpoints

#### Get System Configuration

```http path=null start=null
GET /admin/system/configuration
Authorization: Bearer <admin_jwt_token>
```

**Purpose**: Display all system configuration for admin monitoring and debugging

**Authentication**: Admin role required

**Response**: `SystemConfiguration`

```typescript path=null start=null
{
  "blockchain_network": {
    "network": "testnet", // or "mainnet"
    "node_uri": "https://grpc.testnet.concordium.com:20000",
    "network_id": "testnet"
  },
  
  "smart_contracts": {
    "identity_registry": {
      "contract_address": "1000,0",
      "module_ref": "0x1234567890abcdef...",
      "contract_name": "rwa_identity_registry"
    },
    "carbon_credits": {
      "contract_address": "2000,0", 
      "module_ref": "0x2345678901bcdef0...",
      "contract_name": "rwa_carbon_credits"
    },
    "security_mint_fund": {
      "contract_address": "3000,0",
      "module_ref": "0x3456789012cdef01...",
      "contract_name": "security_mint_fund"
    }
  },
  
  "database_configuration": {
    "database_url": "postgresql://***:***@localhost:5432/upwood_db", // Masked credentials
    "max_connections": 10,
    "connection_timeout": "30s"
  },
  
  "api_configuration": {
    "web_server_addr": "0.0.0.0:3000",
    "cors_enabled": true,
    "jwt_secret": "***", // Masked
    "rate_limiting": {
      "requests_per_minute": 100,
      "burst_size": 20
    }
  },
  
  "aws_services": {
    "region": "us-east-1",
    "cognito_user_pool_id": "us-east-1_ABC123DEF",
    "cognito_client_id": "***", // Masked
    "s3_bucket": "upwood-documents-dev"
  },
  
  "event_processing": {
    "batch_size": 100,
    "processing_interval": "5s",
    "retry_attempts": 3,
    "dead_letter_enabled": true
  },
  
  "system_info": {
    "version": "1.0.0",
    "build_date": "2024-01-15T10:00:00Z",
    "environment": "development", // "development", "staging", "production"
    "uptime": "72h 34m 12s",
    "last_restart": "2024-03-08T14:30:00Z"
  }
}
```

#### Get Contract Details

```http path=null start=null
GET /admin/system/contracts/{contract_type}
Authorization: Bearer <admin_jwt_token>
```

**Purpose**: Get detailed information about specific contract type

**Path Parameters**:
- `contract_type`: "identity-registry" | "carbon-credits" | "security-mint-fund"

**Authentication**: Admin role required

**Response**: `ContractDetails`

```typescript path=null start=null
{
  "contract_type": "identity-registry",
  "contract_address": "1000,0",
  "module_ref": "0x1234567890abcdef...",
  "contract_name": "rwa_identity_registry",
  "deployment_info": {
    "deployed_at": "2024-01-15T10:00:00Z",
    "deployed_by": "4owvMHZAXXX...",
    "deployment_transaction": "0xabc123..."
  },
  "current_status": {
    "is_active": true,
    "last_interaction": "2024-03-10T14:30:00Z",
    "total_transactions": 1250,
    "current_agents": 3
  }
}
```

#### Get Environment Variables

```http path=null start=null
GET /admin/system/environment
Authorization: Bearer <admin_jwt_token>
```

**Purpose**: Display non-sensitive environment variables for debugging

**Authentication**: Admin role required

**Response**: `EnvironmentVariables`

```typescript path=null start=null
{
  "blockchain": {
    "CONCORDIUM_NODE_URI": "https://grpc.testnet.concordium.com:20000",
    "NETWORK": "testnet"
  },
  
  "contracts": {
    "IDENTITY_REGISTRY_CONTRACT": "1000,0",
    "CARBON_CREDIT_CONTRACT": "2000,0",
    "SECURITY_MINT_FUND_CONTRACT": "3000,0"
  },
  
  "api_settings": {
    "WEB_SERVER_ADDR": "0.0.0.0:3000",
    "CORS_ENABLED": "true",
    "RATE_LIMIT_REQUESTS_PER_MINUTE": "100"
  },
  
  "database": {
    "DATABASE_URL": "postgresql://***:***@localhost:5432/upwood_db", // Credentials masked
    "DATABASE_MAX_CONNECTIONS": "10"
  },
  
  "aws": {
    "AWS_REGION": "us-east-1",
    "COGNITO_USER_POOL_ID": "us-east-1_ABC123DEF",
    "S3_BUCKET_NAME": "upwood-documents-dev"
  },
  
  // Note: Sensitive variables like secrets, passwords are excluded
  "excluded_variables": [
    "JWT_SECRET",
    "AWS_ACCESS_KEY_ID",
    "AWS_SECRET_ACCESS_KEY",
    "COGNITO_CLIENT_SECRET",
    "DATABASE_PASSWORD"
  ]
}
```

#### Get System Health

```http path=null start=null
GET /admin/system/health
Authorization: Bearer <admin_jwt_token>
```

**Purpose**: System health check and status information

**Authentication**: Admin role required

**Response**: `SystemHealth`

```typescript path=null start=null
{
  "overall_status": "healthy", // "healthy", "degraded", "unhealthy"
  
  "services": {
    "database": {
      "status": "healthy",
      "response_time_ms": 12,
      "active_connections": 3,
      "last_check": "2024-03-10T15:30:00Z"
    },
    "blockchain_node": {
      "status": "healthy",
      "response_time_ms": 45,
      "last_block_height": 123456,
      "last_check": "2024-03-10T15:30:00Z"
    },
    "aws_services": {
      "status": "healthy",
      "cognito_available": true,
      "s3_available": true,
      "last_check": "2024-03-10T15:30:00Z"
    },
    "event_processor": {
      "status": "healthy",
      "last_processed_block": 123450,
      "processing_lag_blocks": 6,
      "last_check": "2024-03-10T15:30:00Z"
    }
  },
  
  "system_metrics": {
    "memory_usage_mb": 256,
    "cpu_usage_percent": 15.3,
    "disk_usage_percent": 42.1,
    "uptime_seconds": 261252
  }
}
```

## Backend Implementation

### Configuration Service

```rust path=null start=null
impl SystemConfigurationService {
    pub fn get_system_configuration(
        config: &SystemConfig
    ) -> SystemConfiguration {
        SystemConfiguration {
            blockchain_network: BlockchainNetworkConfig {
                network: config.network.clone(),
                node_uri: config.concordium_node_uri.clone(),
                network_id: config.network.clone(),
            },
            
            smart_contracts: SmartContractsConfig {
                identity_registry: ContractInfo {
                    contract_address: config.identity_registry_contract.to_string(),
                    module_ref: config.identity_registry_module_ref.clone(),
                    contract_name: "rwa_identity_registry".to_string(),
                },
                carbon_credits: ContractInfo {
                    contract_address: config.carbon_credit_contract.to_string(),
                    module_ref: config.carbon_credit_module_ref.clone(),
                    contract_name: "rwa_carbon_credits".to_string(),
                },
                security_mint_fund: ContractInfo {
                    contract_address: config.security_mint_fund_contract.to_string(),
                    module_ref: config.security_mint_fund_module_ref.clone(),
                    contract_name: "security_mint_fund".to_string(),
                },
            },
            
            database_configuration: DatabaseConfig {
                database_url: mask_credentials(&config.database_url),
                max_connections: config.database_max_connections,
                connection_timeout: format!("{}s", config.database_timeout_seconds),
            },
            
            // ... other config sections
        }
    }

    pub fn get_contract_details(
        config: &SystemConfig,
        contract_type: &str,
        conn: &mut PgConnection
    ) -> Result<ContractDetails, ServiceError> {
        match contract_type {
            "identity-registry" => {
                let contract_address = config.identity_registry_contract;
                let deployment_info = get_deployment_info(conn, contract_address)?;
                let current_status = get_contract_status(conn, contract_address)?;
                
                Ok(ContractDetails {
                    contract_type: "identity-registry".to_string(),
                    contract_address: contract_address.to_string(),
                    module_ref: config.identity_registry_module_ref.clone(),
                    contract_name: "rwa_identity_registry".to_string(),
                    deployment_info,
                    current_status,
                })
            },
            // ... other contract types
            _ => Err(ServiceError::InvalidContractType)
        }
    }

    pub fn get_environment_variables() -> EnvironmentVariables {
        let mut env_vars = EnvironmentVariables::default();
        
        // Only include non-sensitive environment variables
        let safe_vars = vec![
            "CONCORDIUM_NODE_URI",
            "NETWORK",
            "WEB_SERVER_ADDR",
            "CORS_ENABLED",
            "AWS_REGION",
            "COGNITO_USER_POOL_ID",
            "S3_BUCKET_NAME",
            // ... other safe variables
        ];

        for var_name in safe_vars {
            if let Ok(value) = std::env::var(var_name) {
                let masked_value = if var_name.contains("URL") && value.contains("://") {
                    mask_credentials(&value)
                } else {
                    value
                };
                env_vars.add_variable(var_name, masked_value);
            }
        }

        env_vars
    }
}

// Utility function to mask sensitive information in URLs
fn mask_credentials(url: &str) -> String {
    if let Ok(parsed) = url::Url::parse(url) {
        if parsed.username() != "" || parsed.password().is_some() {
            let mut masked = parsed.clone();
            masked.set_username("***").ok();
            masked.set_password(Some("***")).ok();
            masked.to_string()
        } else {
            url.to_string()
        }
    } else {
        url.to_string()
    }
}
```

### Health Check Service

```rust path=null start=null
impl HealthCheckService {
    pub async fn get_system_health(
        config: &SystemConfig,
        conn: &mut PgConnection
    ) -> SystemHealth {
        let mut services = HashMap::new();
        
        // Database health check
        let db_health = check_database_health(conn).await;
        services.insert("database".to_string(), db_health);
        
        // Blockchain node health check
        let blockchain_health = check_blockchain_health(config).await;
        services.insert("blockchain_node".to_string(), blockchain_health);
        
        // AWS services health check
        let aws_health = check_aws_services_health(config).await;
        services.insert("aws_services".to_string(), aws_health);
        
        // Event processor health check
        let processor_health = check_event_processor_health(conn).await;
        services.insert("event_processor".to_string(), processor_health);
        
        // Determine overall status
        let overall_status = determine_overall_status(&services);
        
        SystemHealth {
            overall_status,
            services,
            system_metrics: get_system_metrics(),
        }
    }

    async fn check_database_health(conn: &mut PgConnection) -> ServiceHealth {
        let start_time = std::time::Instant::now();
        
        match diesel::sql_query("SELECT 1").execute(conn) {
            Ok(_) => {
                let response_time = start_time.elapsed().as_millis();
                ServiceHealth {
                    status: "healthy".to_string(),
                    response_time_ms: response_time as u32,
                    details: serde_json::json!({
                        "active_connections": get_active_connections(conn),
                    }),
                    last_check: chrono::Utc::now(),
                }
            },
            Err(_) => ServiceHealth {
                status: "unhealthy".to_string(),
                response_time_ms: 0,
                details: serde_json::json!({"error": "Database connection failed"}),
                last_check: chrono::Utc::now(),
            }
        }
    }

    async fn check_blockchain_health(config: &SystemConfig) -> ServiceHealth {
        // Implement blockchain node connectivity check
        // Make a simple query to get latest block height
    }

    async fn check_aws_services_health(config: &SystemConfig) -> ServiceHealth {
        // Check AWS services availability (Cognito, S3)
        // Can use simple AWS SDK calls to verify connectivity
    }
}
```

## Frontend Integration

### Admin Portal System Configuration Section

**System Configuration Dashboard**:
- Overview cards showing system status, version, uptime
- Network and environment information
- Contract addresses and module refs

**Configuration Details View**:
- Tabbed interface:
  - Smart Contracts tab
  - Environment Variables tab
  - System Health tab
  - AWS Configuration tab

**Health Monitoring**:
- Real-time health status indicators
- Service response times
- System metrics (CPU, memory, disk)
- Alert indicators for unhealthy services

### UI Components

**Configuration Cards**:
- Network information
- Contract addresses with copy buttons
- Environment summary
- System status indicators

**Health Status Dashboard**:
- Service status badges (healthy/degraded/unhealthy)
- Response time graphs
- System metrics widgets
- Last check timestamps

**Configuration Tables**:
- Contract information with module refs
- Environment variables (non-sensitive only)
- Service endpoints and addresses

This provides comprehensive system configuration visibility for admin debugging and monitoring purposes.
