## ADDED Requirements

### Requirement: HEX 格式化輸出
系統 SHALL 將計算結果的整數部分轉換為大寫十六進位字串。

#### Scenario: 正整數 HEX 轉換
- **WHEN** 結果為 `255`
- **THEN** HEX 輸出為 `"FF"`

#### Scenario: 浮點數 HEX 轉換
- **WHEN** 結果為 `3.14159`（PI）
- **THEN** HEX 輸出為 `"3"`（取整數部分再轉 hex）

---

### Requirement: DEC 格式化輸出
系統 SHALL 將計算結果以十進位格式輸出：
- 整數結果：顯示經 Bit-Depth 遮罩後的整數
- 浮點結果：顯示完整浮點數

#### Scenario: 整數 DEC 輸出
- **WHEN** 結果為 `255`，Bit-Depth 為 16
- **THEN** DEC 輸出為 `"255"`

#### Scenario: 浮點數 DEC 輸出
- **WHEN** 結果為 PI
- **THEN** DEC 輸出為 `"3.141592653589793"`

---

### Requirement: Bit-Depth 遮罩
系統 SHALL 支援 64/32/16 位元遮罩，將整數部分以 BigInt 方式進行 bit-and 遮罩操作後輸出 HEX 和 DEC 整數值。

#### Scenario: 64-bit 遮罩
- **WHEN** 結果為 `256`，Bit-Depth 為 64
- **THEN** HEX 輸出為 `"100"`，DEC 輸出為 `"256"`

#### Scenario: 16-bit 遮罩（溢位截斷）
- **WHEN** 結果為 `65537`（超過 16-bit 範圍），Bit-Depth 為 16
- **THEN** HEX 輸出為 `"1"`，DEC 輸出為 `"1"`（65537 & 0xFFFF = 1）

#### Scenario: 32-bit 遮罩
- **WHEN** 結果為 `-1`，Bit-Depth 為 32
- **THEN** HEX 輸出為 `"FFFFFFFF"`（二補數表示）
