// Copyright 2022 The Engula Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use async_trait::async_trait;
use futures::stream::Stream;

use crate::Result;

mod journal;
mod master;
mod orchestrator;
mod stream_reader;
mod stream_writer;

pub use master::Master;
pub use stream_reader::Reader as StreamReader;
pub use stream_writer::Writer as StreamWriter;

/// The role of a stream.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Role {
    /// A leader manipulate a stream.
    Leader,
    /// A follower subscribes a stream.
    Follower,
}

/// The role and leader's address of current epoch.
pub trait EpochState {
    fn epoch(&self) -> u64;

    /// The role of the associated stream.
    fn role(&self) -> Role;

    /// The leader of the associated stream.
    fn leader(&self) -> Option<String>;
}

/// A trait of shared journals. Those journal's streams divide time into epochs,
/// and each epoch have at most one producer.
#[async_trait]
pub trait Journal: crate::Journal {
    type EpochState: EpochState;
    type StateStream: Stream<Item = Result<Self::EpochState>>;

    /// Return the current epoch state of the specified stream.
    async fn current_state(&self, stream_name: &str) -> Result<Self::EpochState>;

    /// Return a endless stream which returns a new epoch state once the
    /// associated stream enters a new epoch.
    async fn subscribe_status(&self, stream_name: &str) -> Self::StateStream;
}
