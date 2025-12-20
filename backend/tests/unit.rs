#[cfg(test)]
mod tests {
    use pocker_back::base::casino;

    #[tokio::test]
    async fn set_a_table_then_add_some_players() {
        let table_id = casino::set_a_table().await;

        let player1_id = casino::add_player_to_table(&table_id).await.unwrap();
        let player2_id = casino::add_player_to_table(&table_id).await.unwrap();

        let players: Vec<String> = casino::get_table_players(&table_id)
            .await
            .unwrap()
            .iter()
            .map(|p| p.id.clone())
            .collect();

        assert_eq!(table_id.len(), 12);
        assert_eq!(player1_id.len(), 8);
        assert_eq!(players, vec![player1_id, player2_id]);
    }
}
