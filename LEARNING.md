# Rust 学習記録

## 概要

Rustlings 完走後、Rust の理解をより実践的なものにするため、Unix/Linux の基本コマンドを Rust で自作する学習を開始した。

基本的な進め方は以下の通り。

1. 問題・仕様を確認する
2. 解答を見ずに自分で設計する
3. 自分で実装する
4. コンパイル・動作確認する
5. AI とコードレビューする
6. 必要に応じて標準ライブラリや Trait を調査する
7. より Rust らしい実装方法を検討する

基本方針は、**「不思議だと思うから、調べる。」**。

便利なライブラリですぐ解決するのではなく、まず標準ライブラリを使って自分で実装し、Rust の仕組みを理解することを優先する。

---

# 1. echo

## 目的

最初のコマンド実装。コマンドライン引数を受け取り、その内容を標準出力へ表示する。

## 主に使用した要素

```rust
std::env::args()
Vec<String>
Iterator
collect()
join()
print!()
println!()
```

## 学んだこと

`args[1]` のような位置指定だけでなく、`env::args().skip(1)` でプログラム名を飛ばして残りの引数を処理する方法や、`texts.join(" ")` で文字列を連結する方法を学んだ。

本の解答では `clap` が使用されていたが、引数解析そのものを学ぶため、まず `std::env::args()` を使って自力で実装する方針を選んだ。

### ポイント

**便利なクレートを使用する前に、そのクレートが何を肩代わりしているのか理解する。**

---

# 2. cat

## 目的

ファイルを開き、その内容を標準出力へ表示する。

## 主に使用した要素

```rust
File
BufReader
BufRead
lines()
Result
?
AsRef<Path>
Deref
PartialEq
```

## 学んだこと

`File::open(file_name)?` がなぜ `&String` でもコンパイルできるのかを調査し、`File::open()` の引数が `P: AsRef<Path>` という制約になっていることを確認した。

`Deref` と `AsRef` は別の仕組みであり、API を理解するときには「どの型へ変換されるか」だけではなく、「関数がどの Trait を要求しているか」を見る必要がある。

型が分からない場合には、`std::any::type_name_of_val(...)` を使って実際の型を確認する方法も利用した。

### ポイント

**Rust では型そのものだけではなく、その型がどの Trait を実装しているのかを見ることが重要。**

---

# 3. head

## 目的

ファイルの先頭部分を表示する。

```text
-n    先頭 N 行
-c    先頭 N バイト
```

## 主に使用した要素

```rust
enum
match
BufRead::lines()
Read
Read::take()
read_to_end()
Write::write_all()
Vec<u8>
usize
isize
```

## Trait とメソッド

`reader.take(...)` を使った際、`take()` は `BufReader` 固有のメソッドではなく `std::io::Read` Trait が提供するメソッドだと学んだ。そのため `use std::io::Read;` が必要だった。

```text
reader.lines()    -> BufRead
reader.take()     -> Read
write_all()       -> Write
```

### head -c と UTF-8

`-c` は文字数ではなくバイト数を扱うため、データを無理に `String` にせず `stdout().write_all(&buffer)` のようにバイト列のまま出力する方法を採用した。

### ポイント

**文字とバイトは別物として扱う。**

---

# 4. wc

## 目的

ファイルの行数、単語数、バイト数などを数える。

```text
wc -l file.txt
wc -w file.txt
wc -c file.txt
wc file.txt
```

複数オプションや複数ファイルへの対応は、Rust 学習として得られるものに対して実装が複雑になるため対象外とした。

## 主に使用した要素

```rust
enum ReadMode
match
BufReader
lines()
split_whitespace()
metadata()
```

## -c の実装

当初は `reader.lines()` で得た各行の `len()` を合計する方法を検討した。しかし `BufRead::lines()` は改行文字を除去するため実際のファイルサイズとは一致しない。

そのため `std::fs::metadata(file_path)?.len()` を使用してファイルサイズを取得する方法へ変更した。

### ポイント

`String::len()`、`BufRead::lines()`、ファイルサイズは、それぞれ別の意味を持つ。

**プログラムが動くだけではなく、対象コマンドの仕様と API の挙動が一致しているか確認する。**

---

# 5. find

## 目的

ファイルシステム上から条件に合うファイルを検索する。

## 主に使用した要素

```rust
Path
PathBuf
String
所有権
借用
```

ファイルパスを単なる文字列ではなく `Path` / `PathBuf` として扱うことを学んだ。これは `cat` で学んだ `AsRef<Path>` にもつながった。

### ポイント

`String` と `Path` / `PathBuf` には、それぞれ異なる役割がある。

---

# 6. uniq

## 目的

連続している同一行をまとめる。

## 主に使用した要素

```rust
Option
Iterator
String
所有権
借用
```

`uniq` では「前の行」を保持して現在行と比較する必要があるため、状態を保持する処理を経験した。

```text
前の行
↓
現在の行と比較
↓
必要なら出力
↓
現在の行を次回用に保存
```

### ポイント

`Option` や所有権・借用を、実際のアルゴリズムの中で考える経験になった。

---

# 7. grep

## 目的

指定されたパターンに一致する行を検索する。

最初は `line.contains(pattern)` を利用した単純な文字列検索から開始した。

## 主に使用した要素

```rust
BufRead
Iterator
Result
Generics
Box<dyn BufRead>
Trait object
```

## 検索処理の抽象化

```rust
fn grep<R: BufRead>(reader: R, ...)
```

のように、ファイルそのものではなく「一行ずつ読めるもの」を受け取る設計へ進んだ。これによってファイルと標準入力を同じ検索処理へ渡せるようになった。

`Box<dyn BufRead>` を通して Trait object と Trait 実装についても学習した。

---

# 8. grep のオプション対応

段階的に次のオプションを追加した。

```text
-n
-i
-v
-w
-c
```

さらに最終段階として正規表現に対応した。

複数オプション同時指定については、コードが複雑になる割に今回の Rust 学習として得られるものが少ないと判断して対象外とした。

---

# 9. grep -c

`-c` は **一致した行数を表示する** 方式を採用した。

全行を走査して一致した行を数え、当初の `contains(...)` から Regex 対応後は `re.is_match(...)` を利用する形へ変更した。

作成したテスト項目について、仕様外としたケースを除いて期待値どおりになることを確認した。

---

# 10. 正規表現

## 目的

C++ や Python でも苦手意識があった正規表現を、あえて Rust で学習することにした。

いきなり `grep` に組み込まず、小さなサンプルプログラムから開始した。

## 学習した代表的な記号

```text
.          任意の1文字
*          直前の要素の0回以上の繰り返し
+          直前の要素の1回以上の繰り返し
?          直前の要素の0回または1回
^          文字列の先頭
$          文字列の末尾
[]         文字クラス
[a-z]+     小文字英字の1文字以上の連続
^[a-z]+$   文字列全体が小文字英字だけで構成されている
```

## Rust Regex

`Cargo.toml` に `regex` を追加し、`Regex::new(&pattern)` で正規表現を生成、`re.is_match(line)` で一致判定する方法を学んだ。

```text
Cargo.toml に regex を追加
↓
Regex::new() のサンプルを作成
↓
is_match() を学習
↓
正規表現テスト R-1 ～ R-7
↓
contains() を is_match() に置き換え
↓
grep クローンへ統合
```

---

# 11. Git / GitHub

作成した各コマンドを Git で管理することにした。

各 Cargo プロジェクト内に個別の `.git` が存在していたため、ネストした Git リポジトリを整理し、親リポジトリで一括管理する構成へ変更した。

最終的に GitHub の `rust-command-clones` リポジトリへ作成したコマンド群を push した。

---

# 学習の流れ

```text
Rustlings
    ↓
echo
    ↓
cat
    ↓
head
    ↓
wc
    ↓
find
    ↓
uniq
    ↓
grep
    ├── 基本検索
    ├── -n
    ├── -i
    ├── -v
    ├── -w
    ├── -c
    └── Regex
```

---

# Rust の知識としての流れ

```text
env::args
    ↓
Vec / Iterator
    ↓
File / BufReader
    ↓
Result / ?
    ↓
AsRef / Deref
    ↓
Read / Write / BufRead
    ↓
Path / PathBuf
    ↓
Option
    ↓
Generics
    ↓
Trait object / Box
    ↓
Regex
```

---

# この学習で確立した方針

## 1. まず自分で設計する

最初から完成コードや解答を見るのではなく、自分で入力、処理、出力、必要なデータ型を考える。

## 2. まず動くものを作る

```text
まず動かす
↓
テストする
↓
レビューする
↓
Rust らしく改善する
```

## 3. コンパイルが通っても疑問は調べる

例えば `File::open(&String)` がなぜ動くのか、`reader.take()` がなぜ `Read` を必要とするのか、`Box<dyn BufRead>` がなぜ `BufRead` として扱えるのかなど、「動いているけれど、なぜ？」と思った部分を重点的に調査する。

## 4. Trait を意識する

Rust のライブラリを理解するときには「この型は何か」だけではなく、「この型はどの Trait を実装しているか」を見る。

今回特に重要だった Trait は以下。

```rust
AsRef
Deref
Read
Write
BufRead
Iterator
```

## 5. 学習効果の低い機能追加はしない

本物の Unix コマンドを完全再現することが目的ではない。

`wc` の複数オプションや `grep` の複数オプション同時指定のように、実装量が増える一方で新しい Rust の知識があまり増えない機能は対象外とする。

**コマンドを完成させることではなく、コマンドを作りながら Rust を理解することが目的。**

---

# 現在地点

2026年10月9日時点で、次のコマンドクローン作成を経験した。

```text
echo
cat
head
wc
find
uniq
grep
```

直近の `grep` では、文字列検索からオプション追加、`BufRead` による抽象化、Generics、Trait object、Regex まで進んだ。

最初の `echo` と比較すると、単に Rust の文法を書く段階から、**型、Trait、所有権、標準ライブラリの設計を考えながら実装する段階**へ進んできた。

---

# 今後

次のコマンドは未定。

今後も基本的には、

```text
自分で設計
↓
自分で実装
↓
テスト
↓
AIレビュー
↓
疑問点を深掘り
↓
必要ならリファクタリング
```

という流れで学習を続ける。
