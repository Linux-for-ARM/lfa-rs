# lfa-rs

Основные функции, структуры и типажи для программ, входящих в состав проекта LFA.

## Модули

- `error` - обработка ошибок. Предоставляет типы `Error`, `ErrorKind` и `Result<T>`;
- `quick_msg` - случайно возникающие сообщения с просьбой доната;
- `traits` - типажи (на данный момент только типаж `Toml` с методами для (де)сериализации TOML-конфигов);
- `utils` - обёртки над функциями из стандартной библиотеки, использующие типы `Error` и `Result` из `lfa_rs`;
- **`prelude`** - сейчас там только импорт типов из `lfa_rs::error`;

## Примеры использования

```rust
use lfa_rs::prelude::*;
use std::process::Command;

let run = Command::new("/bin/rm")
  .arg("-rf")
  .arg("/boot")
  .status() // Возвращает std::io::Result<...>
  // "Конвертируем" std::io::Result в lfa_rs::error::Result
  // и устанавливаем значение ErrorKind на ErrorKind::ExecError
  // (по умолчанию: ErrorKind::Other).
  .map_err(|err| Error::new(err).set_kind(ErrorKind::ExecError))?;

// Ура! Мы удалили каталог /boot!
assert_eq!(run.code(), Some(0));
```
