use interfaces::UpdateGameCommand;
use server;

#[actix_rt::test]
// Should be run when mongodb is running
#[ignore]
async fn create_game_test() {
    let game_title = "test-game".to_owned();
    let game_info = server::create_game(&game_title).await.unwrap();
    assert_eq!(game_info.title, game_title);
    assert_eq!(game_info.questions.len(), 0);

    let new_game_info = server::get_game_info(game_info.id).await.unwrap();
    assert_eq!(game_info.id, new_game_info.id);

    server::update_game(
        &game_info.id,
        &UpdateGameCommand::AddQuestion {
            question: "q1".to_string(),
        },
    )
    .await
    .unwrap();
    let new_game_info = server::get_game_info(game_info.id).await.unwrap();
    assert_eq!(new_game_info.questions[0], "q1");

    server::update_game(
        &game_info.id,
        &UpdateGameCommand::RemoveQuestion {
            question: "q1".to_string(),
        },
    )
    .await
    .unwrap();

    let new_game_info = server::get_game_info(game_info.id).await.unwrap();
    assert_eq!(new_game_info.questions.len(), 0);

    server::update_game(
        &game_info.id,
        &UpdateGameCommand::ChangeTitle {
            title: "new-title!!!".to_string(),
        },
    )
    .await
    .unwrap();

    let new_game_info = server::get_game_info(game_info.id).await.unwrap();
    assert_eq!(new_game_info.title, "new-title!!!");
}
