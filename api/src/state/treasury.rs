use bytemuck::{Pod, Zeroable};
use shank::ShankAccount;

use crate::utils::{impl_account_from_bytes, impl_to_bytes, Discriminator};

use super::AccountDiscriminator;

/// Treasury is a singleton account which manages all program wide variables.
/// It is the mint authority for the Ore token and also the authority of the program-owned token account.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, ShankAccount, Zeroable)]
pub struct Treasury {}

impl Discriminator for Treasury {
    fn discriminator() -> u8 {
        AccountDiscriminator::Treasury.into()
    }
}

impl_to_bytes!(Treasury);
impl_account_from_bytes!(Treasury);
