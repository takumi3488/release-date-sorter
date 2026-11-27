# release-date-sorter

https://rds.onara.boo/

漫画とかで3.5巻とか番外編とかが入ってくるとどれから読めばいいか分からなくなるので、発売日順に表示しつつ進捗を保存できるようにしました。
現状ダンまちと青ブタ（小説）に対応しています。

## 構成

[react-axum-template](https://github.com/takumi3488/react-axum-template) を使用しています。

### フロントエンド

React の SPA を Farm を用いて静的ビルドします。ビルドしたファイルはサーバーサイド側でサーブします。

### サーバーサイド

Rust (Axum) でゆるいクリーンアーキテクチャ風で構成しています。
依存順は以下で、上が下に依存します。

- persistence (db接続とか)
- handler（ルーターに渡すやつ）
- usecase（ビジネスロジック）
- domain（ドメインモデルとかリポジトリのインタフェースとか）
