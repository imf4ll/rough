<div align="center">
    <img src="./assets/showcase.gif" />
    <h3>A GTK based app launcher</h3>
</div>

## ❗️ Install:

**This program have Linux as target system, we don't give support for any other system**

```bash
git clone https://github.com/z3oxs/rough.git && cd rough
make install
```

&nbsp;

or check [releases section](https://github.com/z3oxs/rough/releases/)

## ❗️ Update:
```bash
git clone https://github.com/z3oxs/rough.git && cd rough
make update
```

&nbsp;

Or, if you maintain the source code
```bash
git pull origin master
make update
```

&nbsp;
## 🚀 Usage:
Running on app-only mode (This is the default mode):
```bash
rough
```

On this mode, you can only run apps, without any shell support

&nbsp;

Running on shell mode:
```bash
rough --shell
```

On this mode, you can run apps and shell commands directly

&nbsp;
<div align="center">

### Available options
| Option | Description |
|--------|-------------|
| -s/--shell | Run on shell mode |

</div>

## ⚙️ Configuration:

Default template: "config.json"
```json
{
    "window_width": 600,
    "box_height": 200
}
```

&nbsp;
<div align="center">

### Available options:
| Option | Description |
|--------|-------------|
| window_width | Width of the main window |
| box_height | Height of the scrollable box |

</div>
