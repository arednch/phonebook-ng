# Phonebook

Experimental version of an AREDN Phonebook implementation in Rust.

## Running

Locally

```bash
cargo run -- --sources ~/AREDN_PhonebookV2.csv
cargo run -- \
  --sources "http://remotehost/filerepo/Phonebook/AREDN_PhonebookV2.csv" \
  --path ~/test.xml \
  --sysinfo ~/sysinfo.json
```

Compile

```bash
cargo build --profile minsize
```
