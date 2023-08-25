# serial-logger

Serial Logger is a utility for logging serial ports

```
Usage: serial-logger [--print] [-h|--help] [-b|--baud=NUM] [--flow-control=n|s|h] [--data-bits=5|6|7|8] [--parity=n|o|e] [--stop-bits=1|2] [-t|--timeout=NUM] [-s|--buffer-size=NUM] [-w|--windows-line-ending] [-l|--log=LOG_FILE_NAME] [--port=SERIAL_PORT_NAME] SERIAL_PORT_NAME

--help: Prints this message
--print: Prints out all available serial ports

--baud: The baud rate to use for the serial port - Default: 115_200
--flow-control: How to handle flow control, n: None, s: Software, h: Hardware - Accepted Values: [n,s,h] Default: Software
--data-bits: How many data bits - Accepted Values: [5,6,7,8] Default: 8
--parity: Parity checking modes, n: None, o: Odd, e: Even - Accepted Values: [n,o,e] Default: None
--stop-bits: Number of Stop Bits - Accepted Values: [1,2] Default: 1
--timeout: Set the amount of time to wait to receive data before timing out - Unit: Seconds, Default: 1
--buffer-size: How large to make the `line` buffer, this should roughly match to the maximum amount output by a single printf, not the size of a single line - Default: 100000
--windows-line-ending: if this is present, when sending through the serial port it will interpret newlines as '\r\n' instead of just '\n' - Default off
--log: The path to a log file - Optional

--port: Will be used instead of the positional argument if defined
```
