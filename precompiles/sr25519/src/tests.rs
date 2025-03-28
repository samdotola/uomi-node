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

use hex_literal::hex;

use crate::mock::*;

use precompile_utils::testing::*;
use sp_core::{sr25519, Pair};

fn precompiles() -> TestPrecompileSet<Runtime> {
    PrecompilesValue::get()
}

#[test]
fn wrong_signature_length_returns_false() {
    ExtBuilder::default().build().execute_with(|| {
        let pair = sr25519::Pair::from_seed(b"12345678901234567890123456789012");
        let public = pair.public();
        let signature = hex!["0042"];
        let message = hex!["00"];

        precompiles()
            .prepare_test(
                TestAccount::Alice,
                PRECOMPILE_ADDRESS,
                PrecompileCall::verify {
                    public: public.into(),
                    signature: signature.into(),
                    message: message.into(),
                },
            )
            .expect_no_logs()
            .execute_returns(false);
    });
}

#[test]
fn bad_signature_returns_false() {
    ExtBuilder::default().build().execute_with(|| {
        let pair = sr25519::Pair::from_seed(b"12345678901234567890123456789012");
        let public = pair.public();
        let message = hex!("2f8c6129d816cf51c374bc7f08c3e63ed156cf78aefb4a6550d97b87997977ee00000000000000000200d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a4500000000000000");
        let signature = pair.sign(&message[..]);
        assert!(sr25519::Pair::verify(&signature, &message[..], &public));

        let bad_message = hex!["00"];

        precompiles()
            .prepare_test(
                TestAccount::Alice,
                PRECOMPILE_ADDRESS,
                PrecompileCall::verify {
                    public: public.into(),
                    signature: <sr25519::Signature as AsRef<[u8]>>::as_ref(&signature).into(),
                    message: bad_message.into(),
                },
            )
            .expect_no_logs()
            .execute_returns(false);
    });
}

#[test]
fn substrate_test_vector_works() {
    ExtBuilder::default().build().execute_with(|| {
        let pair = sr25519::Pair::from_seed(b"12345678901234567890123456789012");
        let public = pair.public();
        assert_eq!(
            public,
            sr25519::Public::from_raw(hex!(
                "741c08a06f41c596608f6774259bd9043304adfa5d3eea62760bd9be97634d63"
            ))
        );
        let message = hex!("2f8c6129d816cf51c374bc7f08c3e63ed156cf78aefb4a6550d97b87997977ee00000000000000000200d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a4500000000000000");
        let signature = pair.sign(&message[..]);
        assert!(sr25519::Pair::verify(&signature, &message[..], &public));

        precompiles()
            .prepare_test(
                TestAccount::Alice,
                PRECOMPILE_ADDRESS,
                PrecompileCall::verify {
                    public: public.into(),
                    signature: <sr25519::Signature as AsRef<[u8]>>::as_ref(&signature).into(),
                    message: message.into(),
                },
            )
            .expect_no_logs()
            .execute_returns(true);
    });
}
