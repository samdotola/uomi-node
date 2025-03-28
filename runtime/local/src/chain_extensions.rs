// This file is part of Uomi.

// Copyright (C) Uomi.
// SPDX-License-Identifier: GPL-3.0-or-later

// Uomi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Uomi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Uomi. If not, see <http://www.gnu.org/licenses/>.

use super::{Runtime, UnifiedAccounts};

/// Registered WASM contracts chain extensions.
pub use pallet_chain_extension_assets::AssetsExtension;
use pallet_contracts::chain_extension::RegisteredChainExtension;

pub use pallet_chain_extension_unified_accounts::UnifiedAccountsExtension;

// Following impls defines chain extension IDs.
impl RegisteredChainExtension<Runtime> for AssetsExtension<Runtime> {
    const ID: u16 = 02;
}

impl RegisteredChainExtension<Runtime> for UnifiedAccountsExtension<Runtime, UnifiedAccounts> {
    const ID: u16 = 03;
}

pub type LocalChainExtensions<Runtime, UnifiedAccounts> = (
    AssetsExtension<Runtime>,
    UnifiedAccountsExtension<Runtime, UnifiedAccounts>,
);
