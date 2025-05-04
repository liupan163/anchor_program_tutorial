use anchor_lang::prelude::Pubkey;
use indexmap::IndexMap;

#[macro_export]
macro_rules! get_struct_values {
    ($struct:expr, $($property: ident),+) => {{
        ($(
            $struct.$property,
        )+)
    }};
}

#[macro_export]
macro_rules! get_then_update_id {
    ($struct:expr, $property: ident) => {{
        let current_id = $struct.$property;
        $struct.$property = current_id.checked_add(1).or(Some(1)).unwrap();
        current_id
    }};
}

#[macro_export]
macro_rules! validate {
        ($assert:expr, $err:expr) => {{
            if ($assert) {
                Ok(())
            } else {
                let error_code: ErrorCode = $err;
                msg!("Error {} thrown at {}:{}", error_code, file!(), line!());
                Err(error_code)
            }
        }};
        ($assert:expr, $err:expr, $($arg:tt)+) => {{
        if ($assert) {
            Ok(())
        } else {
            let error_code: ErrorCode = $err;
            msg!("Error {} thrown at {}:{}", error_code, file!(), line!());
            msg!($($arg)*);
            Err(error_code)
        }
    }};
}

#[macro_export]
macro_rules! dlog {
    ($($variable: expr),+) => {{
        $(
            msg!("{}: {}", stringify!($variable), $variable);
        )+
    }};
    ($($arg:tt)+) => {{
            #[cfg(not(feature = "mainnet-beta"))]
            msg!($($arg)+);
    }};
}

#[macro_export]
macro_rules! load_mut {
    ($account_loader:expr) => {{
        $account_loader.load_mut().map_err(|e| {
            msg!("e {:?}", e);
            let error_code = ErrorCode::UnableToLoadAccountLoader;
            msg!("Error {} thrown at {}:{}", error_code, file!(), line!());
            error_code
        })
    }};
}

#[macro_export]
macro_rules! load {
    ($account_loader:expr) => {{
        $account_loader.load().map_err(|_| {
            let error_code = ErrorCode::UnableToLoadAccountLoader;
            msg!("Error {} thrown at {}:{}", error_code, file!(), line!());
            error_code
        })
    }};
}

#[macro_export]
macro_rules! safe_increment {
    ($struct:expr, $value:expr) => {{
        $struct = $struct.checked_add($value).ok_or_else(math_error!())?
    }};
}

#[macro_export]
macro_rules! safe_decrement {
    ($struct:expr, $value:expr) => {{
        $struct = $struct.checked_sub($value).ok_or_else(math_error!())?
    }};
}

/// Calculate the sha256 digest of anchor encoded `struct`
#[macro_export]
macro_rules! digest_struct {
    ($struct:expr) => {
        solana_program::hash::hash(&$struct.try_to_vec().unwrap()).to_bytes()
    };
}

/// Calculate the hexified sha256 digest of anchor encoded `struct`
#[macro_export]
macro_rules! digest_struct_hex {
    ($struct:expr) => {{
        hex::encode(digest_struct!($struct)).into_bytes()
    }};
}


/// Type of the replicated value
/// These are labels for values in a record that is associated with `Pubkey`
#[derive(PartialEq, Hash, Eq, Clone, Debug)]
pub enum CrdsValueLabel {
    LegacyContactInfo(Pubkey),
    SnapshotHashes(Pubkey),
    ContactInfo(Pubkey),
    RestartLastVotedForkSlots(Pubkey),
    RestartHeaviestFork(Pubkey),
}

/// This structure stores some local metadata associated with the CrdsValue
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VersionedCrdsValue {
    /// Ordinal index indicating insert order.
    ordinal: u64,
    /// local time when updated
    pub(crate) local_timestamp: u64,
    /// None -> value upserted by GossipRoute::{LocalMessage,PullRequest}
    /// Some(0) -> value upserted by GossipRoute::PullResponse
    /// Some(k) if k > 0 -> value upserted by GossipRoute::PushMessage w/ k - 1 push duplicates
    num_push_recv: Option<u8>,
}

type CrdsTable = IndexMap<CrdsValueLabel, VersionedCrdsValue>;
/// Represents types which can be looked up from crds table given a key. e.g.
///   CrdsValueLabel -> VersionedCrdsValue, CrdsValue, CrdsData
///   Pubkey -> ContactInfo, LowestSlot, SnapshotHashes, ...
pub trait CrdsEntry<'a, 'b>: Sized {
    type Key; // Lookup key.
    fn get_entry(table: &'a CrdsTable, key: Self::Key) -> Option<Self>;
}
macro_rules! impl_crds_entry (
    // Lookup by CrdsValueLabel.
    ($name:ident, |$entry:ident| $body:expr) => (
        impl<'a, 'b> CrdsEntry<'a, 'b> for &'a $name {
            type Key = &'b CrdsValueLabel;
            fn get_entry(table:&'a CrdsTable, key: Self::Key) -> Option<Self> {
                let $entry = table.get(key);
                $body
            }
        }
    );
    // Lookup by Pubkey.
    ($name:ident, $pat:pat, $expr:expr) => (
        impl<'a, 'b> CrdsEntry<'a, 'b> for &'a $name {
            type Key = Pubkey;
            fn get_entry(table:&'a CrdsTable, key: Self::Key) -> Option<Self> {
                let key = CrdsValueLabel::$name(key);
                match table.get(&key)?.value.data() {
                    $pat => Some($expr),
                    _ => None,
                }
            }
        }
    );
);

// impl_crds_entry!(CrdsValue, |entry| Some(&entry?.value));