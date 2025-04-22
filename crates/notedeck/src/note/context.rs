use enostr::{ClientMessage, NoteId, Pubkey, RelayPool};
use nostrdb::{Note, NoteKey};
use tracing::error;

/// When broadcasting notes, this determines whether to broadcast
/// over the local network via multicast, or globally
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BroadcastContext {
    LocalNetwork,
    Everywhere,
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub enum NoteContextSelection {
    CopyText,
    CopyPubkey,
    CopyNoteId,
    CopyNoteJSON,
    Broadcast(BroadcastContext),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ContextSelection {
    pub note_key: NoteKey,
    pub action: NoteContextSelection,
}

impl NoteContextSelection {
    pub fn process(&self, ui: &mut egui::Ui, note: &Note<'_>, pool: &mut RelayPool) {
        match self {
            NoteContextSelection::Broadcast(context) => {
                tracing::info!("Broadcasting note {}", hex::encode(note.id()));
                match context {
                    BroadcastContext::LocalNetwork => {
                        pool.send_to(&ClientMessage::event(note).unwrap(), "multicast");
                    }

                    BroadcastContext::Everywhere => {
                        pool.send(&ClientMessage::event(note).unwrap());
                    }
                }
            }
            NoteContextSelection::CopyText => {
                ui.ctx().copy_text(note.content().to_string());
            }
            NoteContextSelection::CopyPubkey => {
                if let Some(bech) = Pubkey::new(*note.pubkey()).npub() {
                    ui.ctx().copy_text(bech);
                }
            }
            NoteContextSelection::CopyNoteId => {
                if let Some(bech) = NoteId::new(*note.id()).to_bech() {
                    ui.ctx().copy_text(bech);
                }
            }
            NoteContextSelection::CopyNoteJSON => match note.json() {
                Ok(json) => ui.ctx().copy_text(json),
                Err(err) => error!("error copying note json: {err}"),
            },
        }
    }
}
