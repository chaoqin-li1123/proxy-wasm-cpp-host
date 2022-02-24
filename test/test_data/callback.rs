// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[no_mangle]
pub extern "C" fn run() {
    unsafe {
        callback();
    }
}

static A : i32 = 100000;

#[no_mangle]
pub extern "C" fn run2(val: i32) -> i32 {
    unsafe {
        callback2(val) + A
    }
}

#[no_mangle]
pub extern "C" fn infinite_loop(val: i32) -> i32 {
    let mut count = val;
    loop {
        count += 1;
        if count >= 10 {
            count -= 2;
        }
        if count >= 100 {
            break;
        }
    }
    unsafe {
        callback2(val) + A
    }
}

extern "C" {
    fn callback();
    fn callback2(val: i32) -> i32;
}

