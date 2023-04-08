//! These are macros to make getting very nested fields in the `Editor` struct easier
//! These are macros instead of functions because functions will have to take `&mut self`
//! However, rust doesn't know that you only want a partial borrow instead of borrowing the
//! entire struct which `&mut self` says.  This makes it impossible to do other mutable
//! stuff to the struct because it is already borrowed. Because macros are expanded,
//! this circumvents the problem because it is just like indexing fields by hand and then
//! putting a `&mut` in front of it. This way rust can see that we are only borrowing a
//! part of the struct and not the entire thing.

/// Get the current view and document mutably as a tuple.
/// Returns `(&mut View, &mut Document)`
#[macro_export]
macro_rules! current {
    ($editor:expr, $id:expr) => {{
        let view = $crate::view_mut!($editor, $id);
        let doc = $crate::doc_mut!($editor, view.doc);
        (view, doc)
    }};
}

#[macro_export]
macro_rules! current_ref {
    ($editor:expr, $id:expr) => {{
        let view = $crate::view!($editor, $id);
        let doc = $crate::doc!($editor, view.doc);
        (view, doc)
    }};
}

/// Get the specified document mutably.
/// Returns `&mut Document`
#[macro_export]
macro_rules! doc_mut {
    ($editor:expr, $id:expr) => {{
        $editor.documents.get_mut(&$id).unwrap()
    }};
}

/// Get the specified document mutably.
/// Returns `&Document`
#[macro_export]
macro_rules! doc {
    ($editor:expr, $id:expr) => {{
        $editor.documents.get(&$id).unwrap()
    }};
}

/// Get the specified view mutably.
/// Returns `&mut View`
#[macro_export]
macro_rules! view_mut {
    ($editor:expr, $id:expr) => {{
        $editor.views.get_mut(&$id).unwrap()
    }};
}

/// Get the specified view immutably
/// Returns `&View`
#[macro_export]
macro_rules! view {
    ($editor:expr, $id:expr) => {{
        $editor.views.get(&$id).unwrap()
    }};
}
