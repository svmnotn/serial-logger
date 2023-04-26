# serial-logger

Serial Logger is a utility for logging serial ports

```
Usage: serial-logger [--print] [-h|--help] [-b|--baud=NUM] [-t|--timeout=NUM] [-s|--buffer-size=NUM] [-l|--log=LOG_FILE_NAME] [--port=SERIAL_PORT_NAME] SERIAL_PORT_NAME

--help: Prints this message
--print: Prints out all available serial ports

--baud - Default: 115_200
--timeout - Unit: Seconds, Default: 120
--buffer-size - Default: 100000
--log - Optional

--port: Will be used instead of the positional argument if defined
```
