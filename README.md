# XMODITS python library

Supported formats:
* Impulse Tracker .IT
* Extended Module .XM
* Scream Tracker .S3M
* Amiga Pro Tracker .MOD
* Open ModPlug Tracker .MPTM (Identical to Impulse Tracker sample-wise)
* Unreal Music Container .UMX (Containing the above formats)
# How to use
```python
import xmodits

file = "~/Downloads/music.xm"
folder = "~/Music/samples/"

# Rip samples to folder
xmodits.dump(file, folder)

```
# Required Arguments
|Argument| Meaning|
| --- | --- |
| ```Path``` | Path to a tracker module |
| ```Destination``` | Destination folder for ripped samples |


# Additional Arguments

|Argument| Definition|
| --- | --- |
| ```with_folder``` | Create a new folder for ripped samples.<br> **e.g.** When set to ```True```, ```"drums.it"``` will create ```"drums_it"``` in the destination folder and place those samples there. |
| ```index_padding``` | Set padding.<br > **e.g.** ```"01 - kick.wav"``` --> ```"1 - kick.wav"``` |
| ```index_only``` | Only name samples with a number.<br> **e.g.** ```"09 - HiHat.wav"``` --> ```"09.wav"``` |
| ```index_raw``` | Preserves the internal sample indexing  |
| ```upper``` | Name samples in upper case |
| ```lower``` | Name samples in lower case |
| ```strict``` | **Enabled by default.** <br>Will reject files that don't match the following file extensions:<br>[it, xm, s3m, mod, umx, mptm]|


# Exceptions
They are pretty much self explanitory.

|Exception| Meaning|
| --- | --- |
|```SampleExtraction```| A sample could not be extracted |
|```PartialExtraction```| Not all of the samples could be extracted |
|```TotalExtraction```| None of the samples could be extracted |
| ```UnsupportedFormat```  | A module format was recognized, but its type is not supported |
| ```InvalidModule``` | The file is not a valid tracker module |
| ```EmptyModule``` | The tracker module is valid but it has no samples |
| ```UnrecognizedFileExtension``` | The file extension was not recognized |
| ```NoFormatFound``` | Could not determine a valid format |

# Additional Examples

### Dump samples without names

```python
import xmodits

tracker = "mods/music.xm"
folder  = "samples/"

xmodits.dump(
    tracker,
    folder,

    index_only=True 
)
```
This produces the following output in folder **"samples"**:

```
01.wav
02.wav
03.wav
04.wav
...
15 - vocal.wav
```
### Dump samples without padding the index:

```python
import xmodits

tracker = "mods/music.xm"
folder  = "samples/"

xmodits.dump(
    tracker,
    folder,
    index_padding=0 # or 1, both have the same effect
)
```

Output:
```
1 - hihat.wav
2 - kick.wav
3 - snare.wav
4 - toms.wav
...
15 - vocal.wav
```

Samples stored in tracker modules can have an arbitary index. If you prefer to use this index, include the parameter:

```python
index_raw=True
```

If you're dumping from multiple modules to the same folder, you're guaranteed to have collisions. 

You should include the parameter:

```Python
with_folder=True
```



## development:
https://docs.python.org/3/library/venv.html

Create a python virtual environment in this directory:

```python -m venv dev```

activate virtual environment:

```source ./dev/bin/activate```

install [maturin (crates.io)](https://crates.io/crates/maturin) or from [pipi](https://pypi.org/project/maturin/)

run test library:

```maturin develop```

# License
The xmodits python library is licensed under the LGPLv3
