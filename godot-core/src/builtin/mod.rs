/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! Built-in types like `Vector2`, `GodotString` or `Variant`.

mod macros;

mod arrays;
mod color;
mod others;
mod string;
mod string_name;
mod variant;
mod vector2;
mod vector3;
mod vector4;

pub use arrays::*;
pub use color::*;
pub use others::*;
pub use string::*;
pub use string_name::*;
pub use variant::*;
pub use vector2::*;
pub use vector3::*;
pub use vector4::*;
