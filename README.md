# yrs-minimum
Minimum reproduction of yrs question

Run

```bash
RUST_BACKTRACE=1 cargo test -- --nocapture --test-threads=1
```

to see the test cases.

The test setup has two test cases. The `collab_edit` test case shows behavior that matches what I would expect. The test uses three `yrs::Doc` documents. The first two represent clients editing a document. The third document is a representation of the server state. The `sync_data` function is used to sync data between the clients and the server, using the `TransactionMut::apply_update` function.

The second test case is `collab_edit_with_merge_updates`. In this test case, there are still two `yrs::Doc` documents used to represent the two clients editing a shared document. Unlike the previous test case, the "server" no longer has an instantiated document, but rather `Vec<u8>` containing a merged set of updates from the clients, aggregated using the `yrs::merge_updates_v1` function inside the `sync_data_by_merge` function in the test crate.  In this second test case, the behavior doesn't match the `collab_edit` test case above. Based on the print statements of document contents, it looks like updates start being lost -- and in some cases, the full content of the server-side merge content is cleared. At the end, the two document states shared through the "server" do not match.

My expectation was that the behavior of the two test cases would match, but I'm not sure whether this indicates a bug or simply my misunderstanding of the API.

Output when I run the above test command:

```
    Finished test [unoptimized + debuginfo] target(s) in 0.03s
     Running unittests src/lib.rs (target/debug/deps/yrs_minimum-d14906875ad73702)

running 2 tests
test tests::collab_edit ... 
Collaborative edit with write before sync

Client1 after 1a text insert: 1a
Server after 1a sync: 1a
Client1 after 1a sync: 1a

Client2 after 2a text insert: 2a
Server after 2a sync: 1a2a
Client2 after 2a sync: 1a2a

Client1 after 1b text insert: 1a1b
Server after 1b sync: 1a1b2a
Client1 after 1b sync: 1a1b2a

Client2 after 2b text insert: 1a2a2b
Server after 2b sync: 1a1b2a2b
Client2 after 2b sync: 1a1b2a2b

Client1 after 1c1d text insert: 1a1b2a1c1d
Client2 after 2c2d text insert: 1a1b2a2b2c2d
Server after 1c1d sync: 1a1b2a1c1d2b
Client1 after 1c1d sync: 1a1b2a1c1d2b
Server after 2c2d sync: 1a1b2a1c1d2b2c2d
Client2 after 2c2d sync: 1a1b2a1c1d2b2c2d

Server after final sync sync: 1a1b2a1c1d2b2c2d
Client1 after final sync sync: 1a1b2a1c1d2b2c2d

ok
test tests::collab_edit_with_merge_updates ... 
Collaborative edit with write before sync merge udpates

Client1 after 1a text insert: 1a
Server after 1a sync: 1a
Client1 after 1a sync: 1a

Client2 after 2a text insert: 2a
Server after 2a sync: 2a1a
Client2 after 2a sync: 2a1a

Client1 after 1b text insert: 1a1b
Server after 1b sync: 1a1b
Client1 after 1b sync: 1a1b

Client2 after 2b text insert: 2a1a2b
Server after 2b sync: 
Client2 after 2b sync: 2a1a2b

Client1 after 1c1d text insert: 1a1b1c1d
Client2 after 2c2d text insert: 2a1a2b2c2d
Server after 1c1d sync: 1a1b1c1d
Client1 after 1c1d sync: 1a1b1c1d
Server after 2c2d sync: 
Client2 after 2c2d sync: 2a1a2b2c2d

Server after final sync sync: 1a1b1c1d
Client1 after final sync sync: 1a1b1c1d

thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `"1a1b1c1d"`,
 right: `"2a1a2b2c2d"`', src/lib.rs:187:9
stack backtrace:
   0: rust_begin_unwind
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:142:14
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:181:5
   4: yrs_minimum::tests::collab_edit_with_merge_updates
             at ./src/lib.rs:187:9
   5: yrs_minimum::tests::collab_edit_with_merge_updates::{{closure}}
             at ./src/lib.rs:148:5
   6: core::ops::function::FnOnce::call_once
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/ops/function.rs:248:5
   7: core::ops::function::FnOnce::call_once
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/ops/function.rs:248:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
FAILED

failures:

failures:
    tests::collab_edit_with_merge_updates

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

error: test failed, to rerun pass `--lib`
```
