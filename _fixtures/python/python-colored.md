```python
# exemd-deps: termcolor;version=1.1.0
import sys
from termcolor import colored, cprint

text = colored('Hello, World!', 'red', attrs=['reverse', 'blink'])
print(text)
```
