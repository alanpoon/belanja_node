// Copyright 2018-2019 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

use vergen::{generate_cargo_keys, ConstantsFlags};

const ERROR_MSG: &str = "Failed to generate metadata files";

fn main() {
	generate_cargo_keys(ConstantsFlags::all()).expect(ERROR_MSG);
	println!("cargo:rerun-if-changed=.git/HEAD");
}
