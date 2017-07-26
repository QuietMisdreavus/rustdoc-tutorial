//! # The Rustdoc Field Guide
//!
//! The purpose of this "document" is to describe how a rustdoc-generated documentation site is
//! organized, how to read through it, and some tips for navigating a doc bundle.
//!
//! For information on how to *use* rustdoc, check out "The Rustdoc Book" (link TBD). For
//! information about *how rustdoc works*, check out my post ["a whirlwind tour of rustdoc"][tour].
//! This post/bundle is mainly about what you can do with a doc bundle that's already been made.
//!
//! [tour]: https://quietmisdreavus.net/code/2017/06/21/a-whirlwind-tour-of-rustdoc/
//!
//! ## The Page Layout
//!
//! There are a few elements that appear on every page, regardless of what kind of item the page is
//! about:
//!
//! ### Sidebar
//!
//! The sidebar lists the headers of the page, followed by the sibling items to that item. For
//! crates, this means it lists the other crates documented in the same `cargo doc` command; for
//! anything else it lists the other items in the same module as the item featured on the page.
//! (For module pages, this means the sidebar lists the items in its *parent* module, which can be
//! a bit confusing!)
//!
//! In doc bundles that have included a header image, there will also be a logo at the top of the
//! sidebar, such as the Rust logo in the standard library documentation.
//!
//! ### Search box
//!
//! When rustdoc generates a doc bundle, it includes a "search index" of all the types, modules,
//! functions, and other items in the crates it processed. This allows the search box at the top of
//! every page to look through all these items and return anything that matches. In addition to
//! being able to search for items by name, it can also filter by the kind of item or the signature
//! of a function. Press `?` for the full syntax and for other keyboard shortcuts. (Like pressing
//! `S` to focus the search box so you can start typing from wherever!)
//!
//! ### `[-]` and `[src]` links
//!
//! In addition to including the rendered documentation comments and the public items of the crate,
//! doc bundles also include the source code of the crate itself! Most items include a `[src]`
//! link, either on the top-right of the page or on the right side of an item heading, which links
//! directly to the item's declaration. This way, you can inspect the implementation of a library
//! without having to find its repository. It's not a fully-featured source-code browser, but in
//! many occasions it can point you to the right spot.
//!
//! Similarly, many things on doc pages can be folded up to take less space on the page. These
//! items are indicated with a `[-]` on their left, which can be clicked to toggle the item.
//! There's a matching `[-]` link at the top-right of the page, which toggles everything on the
//! page, to fold up everything and get a concise overview of everything on the page. This works
//! really well for structs with a lot of methods, so you can quickly scan the list to find one you
//! need.
//!
//! ## Auto-generated elements
//!
//! Even without doc comments to render, rustdoc still creates pages and headings for all the
//! public items in the crate. Everything below the next paragraph was put in place by rustdoc
//! reading the top "summary line" of each of these items and writing a link to the full page.
//!
//! At this point, feel free to explore! A lot of elements in a doc bundle are hyperlinked, so
//! click around and see where it takes you!

#![deny(missing_docs)]

pub mod demo_module;

/// An exported struct to demonstrate field, associated function, and trait implementation docs are
/// shown.
pub struct DemoStruct {
    /// A field containing a primitive.
    pub field_one: i32,
    /// A field containing an Option. See how both `Option` and `String` are hyperlinked
    /// separately.
    pub field_two: Option<String>,
}

/// Here we implement a trait for our struct. Note that like regular impl blocks, trait impl blocks
/// can have documentation comments too! This is helpful to describe the implementation details of
/// a particular trait.
///
/// You can document individual methods as well, but leaving those out will just include the
/// "summary" line from the trait itself.
impl DemoTrait for DemoStruct {
    type Qwop = String;

    fn frob(&self) -> usize {
        self.field_one as usize
    }

    fn tare(&self) -> Self::Qwop {
        self.field_two.as_ref().cloned().unwrap_or_else(String::new)
    }
}

/// An exported enum to demonstrate how enum variant docs are shown.
pub enum DemoEnum {
    /// A tuple variant that contains a local type.
    VariantOne(DemoStruct),
    /// A struct variant that contains a few fields.
    VariantTwo {
        /// The name of the item.
        item: String,
        /// How many items there are.
        amount: usize,
    },
}

/// An exported trait to demonstrate the difference between provided and required methods, and to
/// show off associated types.
pub trait DemoTrait {
    /// An associated type; what do we return when taring?
    type Qwop;

    /// A required method, that implementors must write when implementing the trait.
    fn frob(&self) -> usize;

    /// A required method, this time returning an associated type.
    fn tare(&self) -> Self::Qwop;

    /// A provided method, implemented in terms of the required methods.
    fn girp(&self) -> usize {
        self.frob() * 2
    }
}
