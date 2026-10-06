

pub enum Action {
    Mint { to: usize, amount: i128 },
    Transfer { to: usize, amount: i128 },
    Approve { spender: usize, amount: i128 },
    TransferFrom { to: usize, amount: i128 },
}


pub const ACTION_SIZE: usize = 6;
pub const MAX_ACTIONS: usize = 16;
pub const MAX_INPUT_LEN: usize = ACTION_SIZE * MAX_ACTIONS;


impl Action {
    /// Decodes one `ACTION_SIZE`-byte chunk: [kind, target, amount (i32, big-endian)].
    pub fn decode(bytes: &[u8], num_accounts: usize) -> Self {
        let target = (bytes[1] as usize) % num_accounts;
        let amount = i32::from_be_bytes([bytes[2], bytes[3], bytes[4], bytes[5]]) as i128;
        match bytes[0] % 4 {
            0 => Action::Mint { to: target, amount },
            1 => Action::Transfer { to: target, amount },
            2 => Action::Approve { spender: target, amount },
            _ => Action::TransferFrom { to: target, amount },
        }
    }
}
