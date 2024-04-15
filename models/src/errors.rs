//
//  Copyright 2024 Ram Flux, LLC.
//


#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
}
