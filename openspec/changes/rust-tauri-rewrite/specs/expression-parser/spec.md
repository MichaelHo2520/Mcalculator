## ADDED Requirements

### Requirement: Tokenizer 支援所有運算式元素
系統 SHALL 將輸入字串分解為 Token 序列，支援以下 Token 類型：
- 數字字面值（整數、浮點數）
- HEX 字面值（`0xFF` 格式和裸 `FF` 格式）
- 運算子（`+`, `-`, `*`, `/`, `%`）
- 位元運算子（`^`, `|`, `&`）
- 函數名（`sin`, `cos`, `tan`, `log`, `exp`, `sqrt`）
- 常數（`PI`）
- 括號（`(`, `)`）
- 階乘運算子（`!`）

#### Scenario: 基本四則運算 Tokenize
- **WHEN** 輸入 `"2+3*4"`
- **THEN** 產出 Token 序列 `[Num(2), Op(+), Num(3), Op(*), Num(4)]`

#### Scenario: HEX 字面值 0x 格式
- **WHEN** 輸入 `"0xFF+1"`
- **THEN** 產出 Token 序列 `[Hex(0xFF), Op(+), Num(1)]`

#### Scenario: 函數呼叫 Tokenize
- **WHEN** 輸入 `"sin(PI/4)"`
- **THEN** 產出 Token 序列 `[Fn(sin), LParen, Const(PI), Op(/), Num(4), RParen]`

---

### Requirement: Recursive Descent Parser 產生 AST
系統 SHALL 根據數學運算優先順序將 Token 序列解析為 AST（抽象語法樹），優先順序為：
1. 括號（最高）
2. 函數呼叫 / 階乘
3. 一元正負號
4. `*`, `/`, `%`
5. `+`, `-`
6. 位元運算 `&`, `|`, `^`（最低）

#### Scenario: 運算子優先順序
- **WHEN** 解析 `"2+3*4"`
- **THEN** AST 為 `Add(Num(2), Mul(Num(3), Num(4)))`，求值為 14

#### Scenario: 巢狀函數呼叫
- **WHEN** 解析 `"sin(cos(0))"`
- **THEN** AST 為 `Call(sin, Call(cos, Num(0)))`

---

### Requirement: AST Evaluator 安全求值
系統 SHALL 遞迴走訪 AST 並計算結果，回傳 `Result<f64, EvalError>`。系統 SHALL NOT 使用 `eval()` 或任何形式的字串執行。

#### Scenario: 複合運算式求值
- **WHEN** 求值 `"2+sin(PI/4)"`，角度模式為弧度
- **THEN** 結果約為 `2.7071`

#### Scenario: 位元運算求值
- **WHEN** 求值 `"255&15"`
- **THEN** 結果為 `15`

#### Scenario: 不完整運算式容錯
- **WHEN** 求值 `"2+"`（不完整）
- **THEN** 回傳 `Err(ParseError)`，不 panic

---

### Requirement: 隱式乘法
系統 SHALL 自動在以下情況插入乘號：
- 數字後接函數名：`2sin(x)` → `2*sin(x)`
- 數字後接常數：`2PI` → `2*PI`
- 數字後接左括號：`5(3)` → `5*(3)`
- 右括號後接左括號：`(2)(3)` → `(2)*(3)`
- 常數後接函數名：`PIsin(1)` → `PI*sin(1)`

#### Scenario: 數字PI隱式乘法
- **WHEN** 解析 `"2PI"`
- **THEN** 等價於 `"2*PI"`，求值約為 `6.2832`

#### Scenario: 數字括號隱式乘法
- **WHEN** 解析 `"5(3+1)"`
- **THEN** 等價於 `"5*(3+1)"`，求值為 `20`

---

### Requirement: 括號自動補全（預覽模式）
系統 SHALL 在預覽模式下自動補全缺少的右括號後再求值。

#### Scenario: 缺少右括號的預覽
- **WHEN** 預覽求值 `"sin(PI/4"`（缺少右括號）
- **THEN** 自動補全為 `"sin(PI/4)"`，回傳正常結果

---

### Requirement: 科學函數
系統 SHALL 支援以下函數（大小寫不敏感）：
- `sin(x)`, `cos(x)`, `tan(x)` — 三角函數
- `log(x)` — 自然對數
- `exp(x)` — e 的 x 次方
- `sqrt(x)` — 平方根
- `n!` — 階乘（n 為非負整數）
- `PI` — 圓周率常數

#### Scenario: 三角函數弧度模式
- **WHEN** 求值 `"sin(PI/2)"`，角度模式為弧度
- **THEN** 結果為 `1.0`

#### Scenario: 三角函數角度模式
- **WHEN** 求值 `"sin(90)"`，角度模式為角度
- **THEN** 結果為 `1.0`

#### Scenario: 階乘
- **WHEN** 求值 `"5!"`
- **THEN** 結果為 `120`

#### Scenario: 負數階乘
- **WHEN** 求值含負數階乘
- **THEN** 回傳 `NaN`
