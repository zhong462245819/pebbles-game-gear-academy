#![no_std]

use core::ptr;
use gstd::{msg, prelude::*};
use pebbles_game_io::*;

static mut PEBBLES_GAME: Option<GameState> = None;

#[no_mangle] // 确保编译器不会改变函数名，以便外部能够按预期访问这些函数。
extern "C" fn init() {
    // 获取初始化参数
    let init_params: PebblesInit = msg::load().expect("Failed to decode PebblesInit");

    // 创建初始游戏状态
    let game_state = GameState {
        pebbles_count: init_params.pebbles_count,
        max_pebbles_per_turn: init_params.max_pebbles_per_turn,
        pebbles_remaining: init_params.pebbles_count,
        difficulty: init_params.difficulty,
        first_player: Player::User,
        winner: None,
    };

    // 设置静态变量 PEBBLES_GAME
    unsafe {
        PEBBLES_GAME = Some(game_state);
    }
}

#[no_mangle] // 确保编译器不会改变函数名，以便外部能够按预期访问这些函数。
extern "C" fn handle() {
    // 从消息中加载 PebblesAction
    let action: PebblesAction = msg::load().expect("Failed to decode PebblesAction");

    // 使用 unsafe 块访问和修改静态变量 PEBBLES_GAME
    unsafe {
        if let Some(mut game) = PEBBLES_GAME.take() {
            // 根据不同的 PebblesAction 执行相应的逻辑
            match action {
                PebblesAction::Turn(pebbles) => {
                    if game.winner.is_none()
                        && pebbles <= game.max_pebbles_per_turn
                        && pebbles <= game.pebbles_remaining
                    {
                        // 更新剩余的石子数
                        game.pebbles_remaining -= pebbles;
                        if game.pebbles_remaining == 0 {
                            game.winner = Some(Player::User);
                            msg::reply(PebblesEvent::Won(Player::User), 0)
                                .expect("Failed to reply with event");
                        } else {
                            let counter_pebbles =
                                game.pebbles_remaining.min(game.max_pebbles_per_turn);
                            game.pebbles_remaining -= counter_pebbles;
                            msg::reply(PebblesEvent::CounterTurn(counter_pebbles), 0)
                                .expect("Failed to reply with event");
                            if game.pebbles_remaining == 0 {
                                game.winner = Some(Player::Program);
                                msg::reply(PebblesEvent::Won(Player::Program), 0)
                                    .expect("Failed to reply with event");
                            }
                        }
                    }
                }
                PebblesAction::GiveUp => {
                    if game.winner.is_none() {
                        game.winner = Some(Player::Program);
                        msg::reply(PebblesEvent::Won(Player::Program), 0)
                            .expect("Failed to reply with event");
                    }
                }
                PebblesAction::Restart {
                    difficulty,
                    pebbles_count,
                    max_pebbles_per_turn,
                } => {
                    // 重置游戏状态
                    game = GameState {
                        pebbles_count,
                        max_pebbles_per_turn,
                        pebbles_remaining: pebbles_count,
                        difficulty,
                        first_player: Player::User,
                        winner: None,
                    };
                }
            }
            // 将修改后的游戏状态重新存入 PEBBLES_GAME
            PEBBLES_GAME = Some(game);
        }
    }
}

#[no_mangle] // 确保编译器不会改变函数名，以便外部能够按预期访问这些函数。
extern "C" fn state() {
    unsafe {
        // 使用 addr_of! 宏获取指向 PEBBLES_GAME 的原始指针，并转换为引用
        if let Some(game) = ptr::addr_of!(PEBBLES_GAME).as_ref() {
            if let Some(game) = game {
                // 回复当前游戏状态
                msg::reply(game, 0).expect("Failed to reply with state");
            }
        }
    }
}
