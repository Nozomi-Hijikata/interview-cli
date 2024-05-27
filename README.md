[![lint and test](https://github.com/Nozomi-Hijikata/ivc/actions/workflows/main.yml/badge.svg)](https://github.com/Nozomi-Hijikata/ivc/actions/workflows/main.yml)

# 面接用CLI(ivc)
この研修修了面談での集計作業を効率化するために作成したCLIツールです。
Rustの勉強のために作ったものでもあるので、粗があるとは思いますが、ご了承ください。

本CLIでは主に下記のようなことができます。
- テンプレートを元にした質問回答の記録
- 記録した配点から合計点の算出

## Install方法
MacOSの場合
```sh
brew tap Nozomi-Hijikata/ivc
brew install ivc
```


# 使い方
## 環境の初期化
初めて使う場合は、下記のコマンドを実行してください。
```sh
ivc init
```

下記のようなプロジェクトが作成されます。

```
.
├── interviews
└── templates
    ├── example.md
    └── output.md
```

- `interviews/`: 研修生ごとの面接結果が記録されます。
- `templates/example.md`: 研修質問事項のテンプレートとなるファイルを設置することができます。
- `templates/output.md`: このファイルは変更しないでください。



## 面接の開始
面接を開始する際は、下記のコマンドを実行してください。
```sh
ivc start -n <研修生名> -t <templateとなるmarkdownファイル名>
```

たとえば、研修生の名前がnozomi_hijikataで、  
テンプレートファイルがrails-tutorial.mdの場合は、下記のように実行してください。
```sh
ivc start -n nozomi_hijikata -t rails-tutorial.md
```

そうすると、下記ファイルが`interviews/`直下に生成されるので、`interviews.md`を編集してください。
```
.
├── interviews
│   └── nozomi_hijikata
│       ├── interviews.md
│       └── output.md
└── templates
    ├── example.md
    └── output.md
```

なお実際に研修修了面談で利用するテンプレートは、別途お渡しします。

## 面接中の作業
`interviews.md`を編集して、回答、点数とコメントを記載していきましょう。

## 面接結果の同期
面接結果を集計するために、下記のコマンドを実行してください。
```sh
ivc sync -n <研修生名>
```

実行すると、`interviews.md`の内容を元に、`output.md`に得点の集計結果が記載されます。

先ほどの例であれば、下記のように実行してください。
```sh
ivc sync -n nozomi_hijikata
```

# その他
オプション等については、下記のコマンドを実行してください。
```
ivc -h
ivc <command> -h
```

# Contributing
問題点や改善案があれば、Issueを作成してください。また、プルリクエストも歓迎します。
