# UMKM4

Aplikasi loyality program sederhana diprogram untuk dapat berjalan di atas jaringan blockchain Nuchain.

Progarm ini adalah pilot projek untuk ujicoba membangun aplikasi di atas jaringan blockchain Nuchain yang ditulis menggunakan eDSL [Ink!](https://substrate.dev/docs/en/knowledgebase/smart-contracts/ink-development) di bahasa pemrograman Rust dan dikompilasi ke WebAssembly.

## Keunggulan

Keunggulan membangun aplikasi di atas jaringan blockchain adalah tidak dibutuhkan *backend server*, semua komputasi dijalankan di jaringan blockchain. Nuchain cukup performan untuk memproses hingga ~5000 transaksi per detik, cukup reliable untuk membangun aplikasi semi *real-time*, yang mana sebelumnya hal ini susah dicapai oleh platform blockchain lama.

Biaya komputasi sangat murah plus menggunakan konsep *on-demand* yang mana pemilik membayar komputasi hanya apabila menggunakan sumber dayanya.

Mudah diakses menggunakan library javascript, memungkinkan implementasi frontends yang luas di berbagai macam platform.

99.99% uptime dan sistem keamanan tinggi.

## Fitur

* Pengelolaan keanggotaan (membership).
* Pemberian point.
* Penggunaan point.

## Test

```bash
$ cargo +nightly test
```

## Build

```bash
$ cargo +nightly contract build
```

NOTES: Make sure you have `cargo-contract` installed.

[] robin

