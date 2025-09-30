# モナディック・パイプライン

`Result` コンビネータを軸に、レコードをパース→検証→拡張→整形する Rust 製モナド的パイプラインのサンプル実装です。短絡的なエラーハンドリングと観測性を重視し、実運用を想定した構成になっています。

## 主な特徴
- 小さな純粋関数 (`parse` / `validate` / `enrich` / `format`) を `Result`/`Option` で合成
- `--min-age` / `--strict-email` / `--age-grouping` など CLI フラグによる柔軟な検証設定
- `stdin` / 単一ファイル / ディレクトリから入力を読み込み、`stdout` またはファイルに出力
- `tracing` + `tracing-subscriber` による人間可読ログと JSON 構造化ログの切り替え
- スパン計測と簡易メトリクス (`lines_total`, `lines_ok`, `lines_err`) をログに出力
- Criterion ベンチマーク、examples、統合テスト・CLI テスト・プロパティテストを同梱

## セットアップ
```bash
cargo build
cargo test
cargo fmt -- --check
cargo clippy -- -D warnings
```

## 使い方
### 単一行を処理する
```bash
echo "Alice,30,alice@example.com" | cargo run -- --in - --strict-email
```

### ディレクトリ内の `.csv` / `.txt` を一括処理し、JSON ログで出力
```bash
cargo run --features json-logs -- --in samples --out out.txt --log json
```

## CLI フラグ一覧
- `--in <PATH|->`: 入力ソース (`-` は標準入力)
- `--out <PATH>`: 出力ファイル（省略時は標準出力）
- `--min-age <u8>`: 許可する最小年齢
- `--strict-email`: 正規表現による厳格なメール検証を有効化
- `--age-grouping <default|fine-grained|wide>`: 年齢グルーピング戦略
- `--log <human|json>`: ログ形式を選択
- `--parallel <N>`: 並列ヒント（現状は情報提供のみで逐次実行）

## テスト戦略
- 単体テスト & プロパティテスト: `src/lib.rs`
- ライブラリ結合テスト: `tests/integration_lib.rs`
- CLI 結合テスト: `tests/integration_cli.rs`
- ベンチマーク: `benches/pipeline_bench.rs`
- 利用例: `examples/basic.rs`

## 観測性
`logging::init_logging` でログ初期化を行い、feature `human-logs` / `json-logs` に応じて人間可読 or JSON を選択できます。`process_lines` のスパンでは行数メトリクスを info / error ログとして出力します。

## ライセンス
MIT ライセンス。詳細は [LICENSE](LICENSE) を参照してください。
