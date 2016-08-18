# Exonum

Тут будет короткое описание

# About

Более подробное описание

# Build dependencies

## System Libraries

### Linux

Для debian based систем понадобятся следующие пакеты:
```
$ apt install build-essential git libsodium-dev
```

### macOS

Прежде всего необходимо установить и настроить homebrew согласно его [инструкции](http://brew.sh/). После чего установить следующие пакеты:
```
$ brew install libsodium
```

_В принципе данную инструкцию можно использовать и для любых linux-based дистрибутивов, если заменить homebrew на [linuxbrew](http://linuxbrew.sh/)_

## Rust

В проекте используется нестабильная ветка, для управления которой существует утилита [rustup](https://www.rustup.rs/).
Для того, чтобы установить ее и заодно нужный для сборки проекта `toolchain` достаточно выполнить следующую команду:
```
$ curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly
```
_Возможны ситуации, когда сборка проекта сломана в текущей версии nightly, если такое происходит, то попробуйте откатиться на более старую версию_

# Build instruction

Сам проект поделен на три части, каждая из которых находится в собственной директории:
 * exonum - ядро
 * sanbox - тестовые приложения
 * timestamping - реализация timestamping сервиса

### Пример команды для сборки exonum:
```
$ cargo build --manifest-path exonum/Cargo.toml
```

### Сборка тестовых приложений:

Пример для сборки узла тестовой сети:
```
$ cargo build --manifest-path sandbox/Cargo.toml --example test_node
```

# Развертка тестовой сети

## Вручную

Для развертки сети используется утилита `test_node`. В первую очередь нужно сгенерировать шаблонный конфиг командой,
в которой параметр `N` - это количество узлов в тестовой сети.
```
$ test_node --config ~/.config/exonum/test.toml generate N
```
Для запуска узла тестовой сети нужно указать его номер (от `0` до `N-1`):
```
$ test_node --config ~/.config/exonum/test.toml run 1
```
Можно задать список узлов, которые он изначально знает (в противном случае он будет знать все).
Номера валидаторов задаются в кавычках через пробел:
```
$ test_node --config ~/.config/exonum/test.toml run 1 --known-peers "1 2 3"
```
Для каждого тестового узла можно задать хранилище, в котором он будет хранить данные.
Если оно не будет задано, то подразумевается, что все данные хранятся в памяти.
```
$ test_node --config ~/.config/exonum/test.toml run --leveldb-path "/var/tmp/exonum/1" 1
```
Опции можно комбинировать, для получения более подробной информации можно вызвать `help`
```
$ test_node --help
```
Или конкретно для команды:
```
$ test_node run --help
```

## Автоматически

Для быстрого разворачивания тестовой сети применяется [supervisord](http://supervisord.org/), который можно установить следуя инструкции на сайте.
Чтобы развернуть сеть в первую очередь нужно собрать бинарник `test_node` и сделать так, чтобы директория, содержащая его, попала в `$PATH`.

Например вот так:
```
$ cargo build --release --manifest-path sandbox/Cargo.toml --example test_node
    Finished release [optimized] target(s) in 0.0 secs
$ export PATH=./sandbox/target/release/examples:$PATH
$ test_node --version
Testnet node 0.1
```

После чего необходимо создать новую конфигурацию:
```
$ ./sandbox/supervisord/create_testnet.sh
```

Сама конфигурация для нашего testnet'а будет создана в директории `/tmp/exonum`.
Если мы хотим изменить параметры сети, то нам достаточно отредактировать файл `/tmp/exonum/testnet.conf`.

После чего просто запускаем `supervisord`
```
supervisord -c ./sandbox/supervisord/etc/supervisord.conf
```

Для управления сетью используется утилита `supervisorctl`, которая запускается следующим образом:
```
supervisorctl -c ./sandbox/supervisord/etc/supervisord.conf
```

В данной утилите есть несколько групп узлов:
 - `simple` - узлы используют хранилище в оперативной памяти
 - `leveldb` - узлы используют `leveldb` хранилище
 - `peers_exchange` - узлы используют `leveldb` хранилище и для них эмулируется необходимость обмениваться пирами.
   По умолчанию не все узлы знают обо всех и им нужно как-то связаться друг с другом для достижения консенсуса

Команда по запуску всей группы:
```
start simple:*
```

Запуск отдельной ноды:
```
start simple:simple_testnet_03
```

Перезапуск ноды:
```
restart simple:simple_testnet_03
```

Более подробную информацию о командах можно найти в `man supervisorctl`
