# Vocal

## Description
Vocal is a terminal app to play music and look cool.

## Installation

### Manual
```bash
git clone https://github.com/Jackboxx/vocal
cd vocal 
make 
make install
```

## Configuration

### color 

##### Description
Color of the audio graph, progress bar, text, and list items.

##### Default Value
```toml
color = 'blue'
```

##### Valid Values
- black/Black
- red/Red
- green/Green
- yellow/Yellow
- blue/Blue
- magenta/Magenta
- cyan/Cyan
- gray/Gray
- lightred/LightRed
- lightgreen/LightGreen
- lightyellow/LightYellow
- lightblue/LightBlue
- lightmagenta/LightMagenta
- lightcyan/LightCyan
- white/White

### highlight_color

##### Description
Color of the selected list item.

##### Default Value
```toml
highlight_color = 'magenta'
```

##### Valid Values
- black/Black
- red/Red
- green/Green
- yellow/Yellow
- blue/Blue
- magenta/Magenta
- cyan/Cyan
- gray/Gray
- lightred/LightRed
- lightgreen/LightGreen
- lightyellow/LightYellow
- lightblue/LightBlue
- lightmagenta/LightMagenta
- lightcyan/LightCyan
- white/White

### audio_directory

##### Description
The directory that is loaded if no values are provided to the `--load` or `--play` flags.
All files in this directory are listed and can be selected to be played.

##### Default Value
```toml
audio_directory = '$HOME/vocal'
```
