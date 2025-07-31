# Tapestry
An example of how to build raw-mode terminal applications (on posix) in different
languages.

![image](https://github.com/user-attachments/assets/73049117-028f-4545-93c8-0e0f967b2daa)

## Features

* Enable/disable raw mode
* Clear and restore existing terminal text
* Show/hide cursor
* Display window size
* Draw window border
* Print out characters as they are input
* Handle SIGWINCH

No dependencies other than libc.

## Usage
```
$ ./tapestry
```

Or, if you want to enable "private mode" escape sequences:
```
$ ./tapestry --allow-private
```
