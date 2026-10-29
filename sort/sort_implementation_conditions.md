# Rust版 `sort` 実装条件

## 1. 基本仕様

```bash
sort [OPTION]... [FILE]...
```

ファイルを読み込み、**1行を1要素として並べ替えて標準出力**する。

例：

```text
orange
apple
banana
```

出力：

```text
apple
banana
orange
```

---

## 2. 対応対象

### Phase 1：通常ソート

```bash
sort file.txt
```

文字列として昇順に並べる。

```rust
lines.sort();
```

最初に、この基本動作を完成させる。

---

### Phase 2：`-r` / `--reverse`

```bash
sort -r file.txt
```

降順に並べる。

```rust
lines.sort_by(|a, b| b.cmp(a));
```

この処理では、クロージャーの引数 `a` と `b` が参照になる理由も確認する。

---

### Phase 3：`-n` / `--numeric-sort`

```bash
sort -n numbers.txt
```

数値として比較して並べる。

入力例：

```text
10
2
100
1
```

通常の文字列比較：

```text
1
10
100
2
```

数値比較：

```text
1
2
10
100
```

この段階では、次の要素を扱う。

- `parse`
- `Result`
- 数値への変換
- `sort_by`
- 比較方法の切り替え

---

### Phase 4：`-u` / `--unique`

```bash
sort -u file.txt
```

ソート後、同一行を1つにする。

入力例：

```text
banana
apple
banana
apple
orange
```

出力：

```text
apple
banana
orange
```

実装には `dedup()` を利用できる。

```rust
lines.sort();
lines.dedup();
```

`uniq` は実装済みなので、ここは復習として扱う。

---

### Phase 5：`-k` / `--key`

指定したフィールドを比較キーとして並べる。

入力例：

```text
yoshino 30
kondo   20
abe     40
```

第2フィールドをキーに比較する場合は、概ね次の形になる。

```rust
lines.sort_by(|a, b| {
    // aから比較キーを取り出す
    // bから比較キーを取り出す
    // キー同士を比較する
});
```

この段階を今回の山場とし、クロージャーを使う意味を確認する。

---

## 3. 入力

当面は、単一ファイルの指定を基本とする。

```bash
sort file1
```

余裕があれば、複数ファイルへ拡張する。

```bash
sort file1 file2
```

標準入力への対応も拡張候補とする。

```bash
cat file.txt | sort
```

---

## 4. 今回の対象外

GNU `sort` の完全再現は狙わず、次の機能は最初の実装対象から外す。

- ロケール依存比較
- 月名ソート
- human-readable数値
- version sort
- 一時ファイルを使った巨大ファイル処理
- 並列ソート
- 複雑なキー範囲指定

**Rustの学習にならない複雑さは切る**方針とする。

---

## 5. 最終目標

次の5つの呼び出しに対応する。

```bash
sort FILE
sort -r FILE
sort -n FILE
sort -u FILE
sort -k N FILE
```

今回の中心テーマは、単純な `sort()` の使用ではなく、次の点とする。

> **オプションによって比較方法をどう切り替えるか**

主に `sort_by()`、クロージャー、`Ordering` を実践的に扱う。
