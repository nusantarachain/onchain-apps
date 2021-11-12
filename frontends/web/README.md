# Web Frontend

Repo berisi kode cara mengakses onchain app Nuchain melalui antarmuka web. Dibuat menggunakan Nuxt.js mode SPA (Single Page App) dan bisa dijalankan secara statis tanpa server.

Pada contoh ini aplikasi web digunakan untuk menjalankan fungsi-fungsi di [umkm4](../../umkm4).

Aplikasi ini juga mendemonstrasikan cara menggunakan external Web3 account manajer seperti [Polkadot Extension](https://polkadot.js.org/extension/), anda perlu menginstall-nya terlebih dahulu untuk mencobanya, terutama apabila ingin melakukan transaksi tertanda (signed transaction).

Pada contoh ini aplikasi menggunakan [Nuchain Testnet](https://dashboard.nuchain.network/?rpc=wss%3A%2F%2Ftestnet.nuchain.riset.tech) sebagai platform smart contract-nya, Anda bisa merubahnya di `nuchain.js` apabila ingin menggunakan di Mainnet.

## Uji coba

Live demo bisa dicoba di [http://umkm4.demo.nuchain.surge.sh](http://umkm4.demo.nuchain.surge.sh/).
Gunakan Contract address `5EfjJKjf57H6VFDStvD5fkGtyNtnjRHdQp8V9K3BxcZ2ezE4` untuk ujicoba, itu adalah address pre-deployed contract di Testnet.

Screenshot:

![Nuchain smart contract](https://i.imgur.com/bj4UQWz.png)


## Build Setup

```bash
# install dependencies
$ yarn install

# serve with hot reload at localhost:3000
$ yarn dev

# build for production and launch server
$ yarn build
$ yarn start

# generate static project
$ yarn generate
```

