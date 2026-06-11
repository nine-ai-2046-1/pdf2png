# pdf2png

> 將 PDF 頁面轉換成 PNG 圖片 — 一個獨立嘅 Rust CLI 工具。

## 背景

大部分 PDF 轉 PNG 嘅工具都需要系統庫（poppler、Ghostscript），或者喺唔同平台
出嘅結果唔一致。`pdf2png` 喺編譯時將 Google 嘅 pdfium 引擎嵌入 binary 入面，
 produce 一個**完全獨立嘅執行檔**，唔需要任何 runtime 依賴。邊度都跑得 —
唔使 `brew install`，唔使 `apt-get`，唔使煩 DLL 問題。

## 功能

- **每頁一張 PNG** — 預設模式，順序命名 `1.png`、`2.png`、...
- **智能分組模式** (`-s`) — 連續同尺寸嘅頁面會垂直合併成一張 PNG
- **可調 DPI** — 預設 300（印刷質素），用 `-d` 改
- **機器可讀輸出** — 完成後喺 stdout 輸出 `{"success":true,"count":N}`，
  方便 shell script 同 CI pipeline 整合
- **完全獨立** — pdfium 喺編譯時嵌入 binary，唔使額外安裝

## 安裝

```bash
# 由 source build（需要 Rust toolchain）
git clone <repo-url> && cd pdf2png
cargo build --release

# Binary 喺 target/release/pdf2png
```

## 快速開始

```bash
# 先建立 output 目錄
mkdir -p output

# 每頁轉一張 PNG
pdf2png -i document.pdf -o output

# 連續同尺寸嘅頁面合併成較少 PNG
pdf2png -i document.pdf -o output -s -y

# 自訂 DPI（越低檔案越細）
pdf2png -i document.pdf -o output -d 150
```

## CLI 參考

| Flag | Long | 描述 | 預設值 |
|------|------|------|--------|
| `-i` | `--input` | 輸入 PDF 檔案路徑 | *（必填）* |
| `-o` | `--output` | PNG 輸出資料夾 | `./output` |
| `-d` | `--dpi` | 渲染 DPI（越高越清，檔案越大） | `300` |
| `-s` | `--smart-group` | 將連續同尺寸頁面合併成一張 PNG | 關 |
| `-y` | `--yes` | 自動確認所有提示（跳過警告） | 關 |

## 智能分組模式

當 `-s` 開咗之後，工具會將**連續**同 pixel 尺寸嘅頁面分組。每組會垂直堆疊
成一張 PNG。

### 方向處理

頁面尺寸會喺指定 DPI 下計算。只有當兩個連續頁面嘅 pixel 寬度同高度**完全一樣**
嗰陣，佢哋先會被分到同一組。一張直向頁面之後跟一張橫向頁面（或者反過嚟），
就會開始新嘅一組。

### 例子

假設一個 5 頁嘅 PDF：

```
第 1 頁：直向  (800×1131 px)
第 2 頁：直向  (800×1131 px)
第 3 頁：橫向  (1131×800 px)
第 4 頁：橫向  (1131×800 px)
第 5 頁：直向  (800×1131 px)
```

智能分組輸出：

```
output/
├── 1.png   ← 第 1+2 頁堆疊（都係直向，同尺寸）
├── 2.png   ← 第 3+4 頁堆疊（都係橫向，同尺寸）
└── 3.png   ← 第 5 頁獨立（直向，同前一組唔同）
```

**關鍵規則：** 非連續嘅同尺寸頁面**唔會**合併。第 5 頁唔會加入第 1 組，
即使佢嘅尺寸一樣 — 文檔順序會被保留。

### 大型 PDF 警告

喺智能分組模式下，如果 PDF 超過 **5 頁**，工具會警告用戶並要求確認先繼續。
用 `-y` 可以跳過呢個提示。

## 項目結構

```
pdf2png/
├── Cargo.toml           # 依賴同 metadata
├── build.rs             # 編譯時下載 pdfium，嵌入做 bytes
├── src/
│   ├── main.rs          # 入口 — 協調整個 pipeline
│   ├── cli.rs           # 參數解析（clap）同驗證
│   ├── renderer.rs      # pdfium-render wrapper：page → DynamicImage
│   ├── grouper.rs       # 連續同尺寸頁面分組
│   ├── stacker.rs       # 垂直圖片堆疊
│   ├── writer.rs        # 順序 PNG 檔案輸出
│   ├── prompt.rs        # 確認提示（>5 頁）
│   └── reporter.rs      # JSON stdout 輸出
└── tests/
    ├── fixtures/        # 測試用嘅 sample PDF
    └── integration_tests.rs
```

## JSON 輸出

成功完成後，`pdf2png` 會喺 stdout 輸出一行 JSON：

```json
{"success":true,"count":3}
```

- `success` — 成功完成嗰陣永遠係 `true`
- `count` — 寫入咗幾多個 PNG 檔案

所有警告同提示都會去 **stderr**，所以 stdout 乾淨，可以直接 shell parsing。

```bash
# 喺 script 入面攞 count
count=$(pdf2png -i doc.pdf -o out -s -y | jq -r '.count')
echo "產生咗 $count 個 PNG"
```
