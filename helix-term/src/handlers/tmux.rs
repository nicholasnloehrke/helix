use helix_event::register_hook;
use helix_view::events::EditorFocusDirectionFailed;
use helix_view::handlers::Handlers;

use std::process::Command;

pub(super) fn register_hooks(_handlers: &Handlers) {
    register_hook!(move |event: &mut EditorFocusDirectionFailed<'_>| {
        log::debug!("EditorFocusDirection failed. Direction={:?}", event.direction);

        let tmux_direction = match event.direction {
            helix_view::tree::Direction::Left => Some("L"),
            helix_view::tree::Direction::Down => Some("D"),
            helix_view::tree::Direction::Up => Some("U"),
            helix_view::tree::Direction::Right => Some("R"),
        };

        if let Some(dir) = tmux_direction {
            let result = Command::new("tmux")
                .arg("select-pane")
                .arg(format!("-{}", dir))
                .status();

            if let Err(e) = result {
                log::error!("Failed to send tmux command: {}", e);
            }
        }

        Ok(())
    });
}
