use yrs::updates::decoder::Decode;
use yrs::{merge_updates_v1, Doc, GetString, ReadTxn, StateVector, Text, Transact, Update};

pub fn doc_write_text(doc: &Doc, value: &str) {
    let text = doc.get_or_insert_text("data");
    let mut txn = doc.transact_mut();
    text.push(&mut txn, value);
    txn.commit();
}

pub fn doc_print_text(doc: &Doc, label: &str) {
    let text = doc.get_or_insert_text("data");
    let mut txn = doc.transact_mut();
    println!("{}: {}", label, text.get_string(&txn));
    txn.commit();
}

pub fn doc_get_text(doc: &Doc) -> String {
    let text = doc.get_or_insert_text("data");
    let mut txn = doc.transact_mut();
    let result = text.get_string(&txn);
    txn.commit();
    result
}

pub fn updates_print_text(update: Vec<u8>, label: &str) {
    let doc = Doc::new();
    let mut txn = doc.transact_mut();
    txn.apply_update(Update::decode_v1(&update).unwrap());
    txn.commit();
    std::mem::drop(txn);

    let text = doc.get_or_insert_text("data");
    let mut txn = doc.transact_mut();
    println!("{}: {}", label, text.get_string(&txn));
    txn.commit();
}

pub fn sync_data(client_doc: &Doc, server_doc: &Doc, client_label: &str, change_label: &str) {
    let mut txn = client_doc.transact_mut();
    let update = txn.encode_diff_v1(&StateVector::default());
    txn.commit();
    std::mem::drop(txn);

    let mut txn = server_doc.transact_mut();
    txn.apply_update(Update::decode_v1(&update).unwrap());
    txn.commit();
    std::mem::drop(txn);
    doc_print_text(server_doc, &format!("Server after {} sync", change_label));

    let mut txn = server_doc.transact_mut();
    let update = txn.encode_diff_v1(&StateVector::default());
    txn.commit();
    std::mem::drop(txn);

    let mut txn = client_doc.transact_mut();
    txn.apply_update(Update::decode_v1(&update).unwrap());
    txn.commit();
    std::mem::drop(txn);
    doc_print_text(
        client_doc,
        &format!("{} after {} sync", client_label, change_label),
    );
}

pub fn sync_data_by_merge(
    client_doc: &Doc,
    server_state: Vec<u8>,
    client_label: &str,
    change_label: &str,
) -> Vec<u8> {
    let mut txn = client_doc.transact_mut();
    let update = txn.encode_diff_v1(&StateVector::default());
    txn.commit();
    std::mem::drop(txn);

    let new_server_state = if server_state.is_empty() {
        update
    } else {
        merge_updates_v1(&[&server_state, &update]).unwrap()
    };
    updates_print_text(
        new_server_state.clone(),
        &format!("Server after {} sync", change_label),
    );

    let mut txn = client_doc.transact_mut();
    txn.apply_update(Update::decode_v1(&new_server_state).unwrap());
    txn.commit();
    std::mem::drop(txn);
    doc_print_text(
        client_doc,
        &format!("{} after {} sync", client_label, change_label),
    );

    new_server_state
}

#[cfg(test)]
mod tests {
    use crate::{doc_get_text, doc_print_text, doc_write_text, sync_data, sync_data_by_merge};
    use yrs::Doc;

    #[test]
    fn collab_edit() {
        println!();
        println!("Collaborative edit with write before sync");
        println!();
        let client1 = Doc::new();
        let client2 = Doc::new();
        let server = Doc::new();

        doc_write_text(&client1, "1a");
        doc_print_text(&client1, "Client1 after 1a text insert");
        sync_data(&client1, &server, "Client1", "1a");
        println!();

        doc_write_text(&client2, "2a");
        doc_print_text(&client2, "Client2 after 2a text insert");
        sync_data(&client2, &server, "Client2", "2a");
        println!();

        doc_write_text(&client1, "1b");
        doc_print_text(&client1, "Client1 after 1b text insert");
        sync_data(&client1, &server, "Client1", "1b");
        println!();

        doc_write_text(&client2, "2b");
        doc_print_text(&client2, "Client2 after 2b text insert");
        sync_data(&client2, &server, "Client2", "2b");
        println!();

        doc_write_text(&client1, "1c1d");
        doc_print_text(&client1, "Client1 after 1c1d text insert");
        doc_write_text(&client2, "2c2d");
        doc_print_text(&client2, "Client2 after 2c2d text insert");
        sync_data(&client1, &server, "Client1", "1c1d");
        sync_data(&client2, &server, "Client2", "2c2d");
        println!();

        sync_data(&client1, &server, "Client1", "final sync");
        println!();

        assert_eq!(doc_get_text(&client1), doc_get_text(&client2));
    }

    #[test]
    fn collab_edit_with_merge_updates() {
        println!();
        println!("Collaborative edit with write before sync merge udpates");
        println!();
        let client1 = Doc::new();
        let client2 = Doc::new();
        let server = Vec::new();

        doc_write_text(&client1, "1a");
        doc_print_text(&client1, "Client1 after 1a text insert");
        let server = sync_data_by_merge(&client1, server, "Client1", "1a");
        println!();

        doc_write_text(&client2, "2a");
        doc_print_text(&client2, "Client2 after 2a text insert");
        let server = sync_data_by_merge(&client2, server, "Client2", "2a");
        println!();

        doc_write_text(&client1, "1b");
        doc_print_text(&client1, "Client1 after 1b text insert");
        let server = sync_data_by_merge(&client1, server, "Client1", "1b");
        println!();

        doc_write_text(&client2, "2b");
        doc_print_text(&client2, "Client2 after 2b text insert");
        let server = sync_data_by_merge(&client2, server, "Client2", "2b");
        println!();

        doc_write_text(&client1, "1c1d");
        doc_print_text(&client1, "Client1 after 1c1d text insert");
        doc_write_text(&client2, "2c2d");
        doc_print_text(&client2, "Client2 after 2c2d text insert");
        let server = sync_data_by_merge(&client1, server, "Client1", "1c1d");
        let server = sync_data_by_merge(&client2, server, "Client2", "2c2d");
        println!();

        let _server = sync_data_by_merge(&client1, server, "Client1", "final sync");
        println!();

        assert_eq!(doc_get_text(&client1), doc_get_text(&client2));
    }
}
