//! Routes the operation message to the appropiate create operation.
use serde::{Serialize, Deserialize};

use crate::routing::enums::Message;

use super::core::{
    create
};


/// The available create operations.
/// 
/// # Variants
/// * `Create` - Create a row in a table. (CreateData is needed and EmptyState is returned)
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum CreateRoutes {
    Create(Message<CreateData, BasicMessage>),
}


/// Accepts a message for a create operation and routes it to the appropiate create operation.
/// 
/// # Arguments
/// * `message` - The message to be routed.
/// 
/// # Returns
/// * `Result<CreateRoutes, String>` - The result of the operation.
pub async fn handle_create_routes(message: CreateRoutes) -> Result<CreateRoutes, String> {
    match message {
        CreateRoutes::Create(message) => {
            let data = message.handle_send()?;
            let _ = create(data.connection_id, data.table_name, data.data).await?;
            let message = Message::<CreateData, BasicMessage>::package_receive(BasicMessage{
                message: "Created".to_string(),
            });
            return Ok(CreateRoutes::Create(message))
        },
    }
}


/// Data representing the CreateData schema
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct CreateData {
    pub connection_id: String,
    pub table_name: String,
    pub data: serde_json::Value,
}


/// Data representing a basic message data schema
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct BasicMessage {
    pub message: String,
}
