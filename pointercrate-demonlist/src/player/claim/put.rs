use crate::{
    error::Result,
    player::{claim::PlayerClaim, DatabasePlayer},
};
use log::info;
use sqlx::PgConnection;

impl DatabasePlayer {
    pub async fn initiate_claim(&self, claimed_by: i32, connection: &mut PgConnection) -> Result<PlayerClaim> {
        // first, clear all claims by the given user
        let result = sqlx::query!("DELETE FROM player_claims WHERE member_id = $1", claimed_by)
            .execute(&mut *connection)
            .await?;

        info!("Cleared {} claims by user with id {}", result.rows_affected(), claimed_by);

        // establish new claim
        sqlx::query!(
            "INSERT INTO player_claims (member_id, player_id) VALUES ($1, $2)",
            claimed_by,
            self.id
        )
        .execute(connection)
        .await?;

        Ok(PlayerClaim {
            user_id: claimed_by,
            player_id: self.id,
            verified: false,
        })
    }
}
