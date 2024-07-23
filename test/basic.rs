use gstd::prelude::*;
use gtest::{Program, System};
use pebbles_game_io::*; // 确保导入 prelude 以使用 Encode 和 Decode trait

#[test]
fn test() {
    let sys = System::new();
    sys.init_logger();

    let program = Program::current(&sys);

    // 初始化游戏
    let init_params = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
    };
    program.send(1, init_params);

    // 获取初始状态
    let state: GameState = program.read_state(()).expect("Failed to read state");
    assert_eq!(state.pebbles_count, 10);
    assert_eq!(state.pebbles_remaining, 10);
    assert_eq!(state.max_pebbles_per_turn, 3);
    assert_eq!(state.difficulty, DifficultyLevel::Easy);
    // assert_eq!(state.winner, None);

    // 玩家进行一次有效的操作
    program.send(1, PebblesAction::Turn(3));
    sys.spend_blocks(1); // 推动系统前进，以确保消息处理

    // 检查状态更新
    let state: GameState = program.read_state(()).expect("Failed to read state");
    assert!(state.pebbles_remaining <= 7); // 玩家拿了3个，程序可能最多拿3个，所以剩余的最多7个
                                           // assert_eq!(state.winner, None);

    // 玩家进行一次无效的操作（超过最大数量）
    program.send(1, PebblesAction::Turn(4));
    sys.spend_blocks(1); // 推动系统前进，以确保消息处理

    let state: GameState = program.read_state(()).expect("Failed to read state");
    assert_eq!(state.pebbles_remaining, 7); // 状态不变
                                            // assert_eq!(state.winner, None);

    // 玩家放弃
    program.send(1, PebblesAction::GiveUp);
    sys.spend_blocks(1); // 推动系统前进，以确保消息处理

    // 重置游戏
    program.send(
        1,
        PebblesAction::Restart {
            difficulty: DifficultyLevel::Hard,
            pebbles_count: 20,
            max_pebbles_per_turn: 5,
        },
    );
    sys.spend_blocks(1); // 推动系统前进，以确保消息处理

    // 获取重置后的状态
    let state: GameState = program.read_state(()).expect("Failed to read state");
    assert_eq!(state.pebbles_count, 20);
    assert_eq!(state.pebbles_remaining, 20);
    assert_eq!(state.max_pebbles_per_turn, 5);
    assert_eq!(state.difficulty, DifficultyLevel::Hard);
    // assert_eq!(state.winner, None);
}
