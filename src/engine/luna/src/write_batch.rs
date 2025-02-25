// Copyright 2021 The Engula Authors.
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

enum Op {
    Put(Vec<u8>, Vec<u8>),
    Delete(Vec<u8>),
}
pub struct WriteBatch {
    mutations: Vec<Op>,
}

impl WriteBatch {
    pub fn put(&mut self, key: &[u8], value: &[u8]) -> &mut Self {
        let op = Op::Put(key.to_owned(), value.to_owned());
        self.mutations.push(op);
        self
    }

    pub fn delete(&mut self, key: &[u8]) -> &mut Self {
        let op = Op::Delete(key.to_owned());
        self.mutations.push(op);
        self
    }
}
