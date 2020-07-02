/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This software may be used and distributed according to the terms of the
 * GNU General Public License version 2.
 */

#![deny(warnings)]
#![feature(never_type)]

use anyhow::Result;
use context::CoreContext;
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use mononoke_types::{ChangesetId, RepositoryId};

mod cache;
mod log;
mod transaction;

pub use bookmarks_types::{
    Bookmark, BookmarkKind, BookmarkName, BookmarkPrefix, BookmarkPrefixRange, Freshness,
};
pub use cache::CachedBookmarks;
pub use log::{BookmarkUpdateLog, BookmarkUpdateLogEntry, BookmarkUpdateReason, BundleReplayData};
pub use transaction::{BookmarkTransaction, BookmarkTransactionError, BookmarkTransactionHook};

pub trait Bookmarks: Send + Sync + 'static {
    /// Get the current value of a bookmark.
    ///
    /// Returns `Some(ChangesetId)` if the bookmark exists, or `None` if doesn't
    fn get(
        &self,
        ctx: CoreContext,
        name: &BookmarkName,
        repoid: RepositoryId,
    ) -> BoxFuture<'static, Result<Option<ChangesetId>>>;

    /// List bookmarks that match certain parameters.
    ///
    /// `prefix` requires that bookmark names begin with a certain prefix.
    ///
    /// `kinds` requires that the bookmark is of a certain kind.
    ///
    /// `limit` limits the total number of bookmarks returned.
    fn list(
        &self,
        ctx: CoreContext,
        repoid: RepositoryId,
        freshness: Freshness,
        prefix: &BookmarkPrefix,
        kinds: &[BookmarkKind],
        limit: u64,
    ) -> BoxStream<'static, Result<(Bookmark, ChangesetId)>>;

    /// Create a transaction to modify bookmarks.
    fn create_transaction(
        &self,
        ctx: CoreContext,
        repoid: RepositoryId,
    ) -> Box<dyn BookmarkTransaction>;
}
