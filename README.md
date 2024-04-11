# pngme
pngme is a command-line tool written in Rust that allows to encode and decode secrete messages in PNG files.

## Writing a message
```
pngme encode <file_path> <chunk_type> <message> <output (optional)>
```

## Reading a message
```
pngme decode <file_path> <chunk_type>
```

## Removing a message
```
pngme remove <file_path> <chunk_type>
```

## Printing file whole file as string
```
pngme print <file_path>
```
