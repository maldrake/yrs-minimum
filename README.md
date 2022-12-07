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
