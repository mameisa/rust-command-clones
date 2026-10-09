rust-command-clones
Rust の学習を目的として、Unix/Linux の基本的なコマンドを Rust で実装しているリポジトリです。
完成したコマンドを再現すること自体よりも、実装を通して Rust の型、所有権、Trait、標準ライブラリの使い方を理解することを目的としています。
Commands
Command	主な学習テーマ
`echo`	コマンドライン引数、`Vec`、Iterator
`cat`	`File`、`BufReader`、`BufRead`、`Result`、`AsRef<Path>`
`head`	`Read`、`Write`、バイト処理、`enum`、`match`
`wc`	行・単語・バイトのカウント、`metadata()`
`find`	`Path`、`PathBuf`、所有権、借用
`uniq`	`Option`、状態保持、所有権、借用
`grep`	Generics、`BufRead`、Trait object、`Box<dyn BufRead>`、Regex
Learning flow
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
Learning policy
このリポジトリでは、次の流れを基本にしています。
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
特に意識しているのは次の点です。
最初から完成した解答を見ず、まず自分で考える
まず動くものを作ってから改善する
コンパイルが通っても「なぜ動くのか」が分からなければ調べる
型だけでなく、その型が実装している Trait を意識する
学習効果に対して複雑さだけが増える機能は、無理に実装しない
> コマンドを完成させることではなく、コマンドを作りながら Rust を理解することが目的。
Topics learned
これまでの実装を通して、主に次の Rust の要素を扱いました。
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
Learning notes
各コマンドを作成する過程で学んだ内容、疑問点、調査した Rust の仕組みについては、LEARNING.md にまとめています。
Repository structure
```text
rust-command-clones/
├── README.md
├── LEARNING.md
├── echo/
├── cat/
├── head/
├── wc/
├── find/
├── uniq/
└── grep/
```
Status
2026-10-09 時点で、`echo`、`cat`、`head`、`wc`、`find`、`uniq`、`grep` の実装を通した学習を完了しています。
次に作成するコマンドは未定です。今後も、未経験の Rust の仕組みを学べる題材を優先して追加していきます。
