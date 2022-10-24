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
| -c/--calc  | Run on calculator mode |

</div>

## ⚙️ Configuration:

Default template: "YOURHOME/.config/rough/config.json"
```json
{
    "window": {
        "width": 600,
        "border_width": 6,
        "border_color": "#252525",
        "opacity": 1.0,
        "background_color": "0, 0, 0",
        "font_color": "#FFFFFF",
        "font": ""
    },
    "container": {
        "max_height": 200
    },
    "textbox": {
        "margin": 0,
        "transparent": false
    },
    "list": {
        "margin_top": 6,
        "transparent": false
    }
}
```

&nbsp;
<div align="center">

## Available options:
### Window
| Option | Description |
|--------|-------------|
| width | Set window width |
| border_width | Set window border width |
| border_color | Set window border color (like in CSS) |
| opacity | If smaller than 1.0, will set a alpha on the background |
| background_color | If transparent enabled, will be the background color (use values between 0 and 1, like "(0.40, 0.42, 0.54)") |
| font_color | Set font color of entire application |
| font | Set font family (Give only the name, ex: '"font": Inter') |

&nbsp;

### Container (ScrolledWindow)
| Option | Description |
|--------|-------------|
| max_height | Set max height of container |

&nbsp;

### Textbox (Entry)
| Option | Description |
|--------|-------------|
| margin | Set margin (all sides) of textbox |
| transparent | If window opacity smaller than 1.0, receive true or false to transparency |

&nbsp;

### List (ListBox)
| Option | Description |
|--------|-------------|
| margin_top | Set margin-top of the list |
| transparent | If window opacity smaller than 1.0, receive true or false to transparency |

</div>
