use std::collections::HashMap;

use enostr::Pubkey;
use notedeck_ui::nip51_set::Nip51SetUiCache;

use crate::deck_state::DeckState;
use crate::login_manager::AcquireKeyState;
use crate::ui::search::SearchQueryState;
use enostr::ProfileState;
use notedeck_ui::media::MediaViewerState;

/// Various state for views
///
/// TODO(jb55): we likely want to encapsulate these better,
/// or at least document where they are used
#[derive(Default)]
pub struct ViewState {
    pub login: AcquireKeyState,
    pub id_to_deck_state: HashMap<egui::Id, DeckState>,
    pub id_state_map: HashMap<egui::Id, AcquireKeyState>,
    pub id_string_map: HashMap<egui::Id, String>,
    pub searches: HashMap<egui::Id, SearchQueryState>,
    pub pubkey_to_profile_state: HashMap<Pubkey, ProfileState>,

    /// Keeps track of what urls we are actively viewing in the
    /// fullscreen media viewier, as well as any other state we want to
    /// keep track of
    pub media_viewer: MediaViewerState,

    /// Keep track of checkbox state of follow pack onboarding
    pub follow_packs: Nip51SetUiCache,
}

impl ViewState {
    pub fn login_mut(&mut self) -> &mut AcquireKeyState {
        &mut self.login
    }
}
